use rusqlite::{params, Connection, Transaction};
use std::path::PathBuf;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new(path: &PathBuf) -> anyhow::Result<Database> {
        log::debug!("Initializing database");

        let conn = Connection::open(path).map_err(|e| anyhow::anyhow!(e))?;
        Ok(Database { conn })
    }

    pub fn init_tables(&self) -> anyhow::Result<()> {
        log::debug!("Initializing database tables");

        // Create the database if it doesn't exist
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS documents (
                id INTEGER PRIMARY KEY,
                path TEXT NOT NULL
            )"#,
            [],
        )?;

        self.conn.execute(
            r#"
                CREATE UNIQUE INDEX IF NOT EXISTS idx_documents_path ON documents(path);
            "#,
            [],
        )?;

        // Create table to store individual pages
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS pages (
                id INTEGER PRIMARY KEY,
                document_id INTEGER NOT NULL,
                page_no INTEGER NOT NULL,
                text TEXT NOT NULL,
                FOREIGN KEY (document_id) REFERENCES documents(id)
            )"#,
            [],
        )?;

        // Create the chunks table that as a foreign key to the documents table
        // And also stores the chunk indices
        self.conn.execute(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY,
                page_id INTEGER NOT NULL,
                chunk_index_start INTEGER NOT NULL,
                chunk_index_end INTEGER NOT NULL,
                FOREIGN KEY (page_id) REFERENCES pages(id)
            )"#,
            [],
        )?;

        Ok(())
    }

    /// Function to get the chunk text from the database given a chunk id
    /// Joins the chunks table with the documents table
    pub fn get_document(&self, chunk_id: u64) -> rusqlite::Result<(String, usize, String)> {
        log::debug!("Getting document text for chunk id {}", chunk_id);
        let mut stmt = self.conn.prepare(
            r#"
            SELECT d.path,
                   p.page_no,
                   substr(p.text, c.chunk_index_start, c.chunk_index_end - c.chunk_index_start) as chunk_text
            FROM pages p
                     INNER JOIN chunks c
                                ON p.id = c.page_id
                     INNER JOIN documents d
                                ON d.id = p.document_id
            WHERE c.id = ?1
        "#,
        )?;
        stmt.query_row(params![chunk_id], |row| {
            let path: String = row.get_unwrap(0);
            let page_no: usize = row.get_unwrap(1);
            let text: String = row.get_unwrap(2);
            Ok((path, page_no, text))
        })
    }

    /// Function to check if a document with the given path exists in the database
    /// Returns true if it does
    pub fn document_exists(&self, path: &str) -> anyhow::Result<bool> {
        let mut stmt = self.conn.prepare(
            r#"
            SELECT EXISTS(
                SELECT 1
                FROM documents
                WHERE path = ?1
            )
        "#,
        )?;
        let exists: u8 = stmt.query_row(params![path], |row| row.get(0))?;

        Ok(exists == 1)
    }
}

/// Function to insert a document into the database
/// And returns the document id
pub fn insert_document(tx: &Transaction, path: &str) -> anyhow::Result<i64> {
    log::debug!("Inserting document into database");

    let mut stmt = tx.prepare("INSERT INTO documents (path) VALUES (?1)")?;
    let id = stmt.insert(params![path])?;
    Ok(id)
}

/// Function to insert a page into the database
/// And returns the page id
pub fn insert_page(
    tx: &Transaction,
    document_id: i64,
    page_no: usize,
    text: &str,
) -> anyhow::Result<i64> {
    log::debug!("Inserting page into database");

    let mut stmt =
        tx.prepare("INSERT INTO pages (document_id, page_no, text) VALUES (?1, ?2, ?3)")?;
    let id = stmt.insert(params![document_id, page_no, text])?;
    Ok(id)
}

/// Function to insert multiple chunks into the database
/// And return the chunk ids in the same order as the chunks
pub fn insert_chunks(
    tx: &Transaction,
    page_id: i64,
    chunks: &[(usize, usize)],
) -> anyhow::Result<Vec<i64>> {
    log::debug!(
        "Inserting {} chunks for page id {} into database",
        chunks.len(),
        page_id
    );

    let ids = {
        let mut stmt = tx.prepare(
            "INSERT INTO chunks (page_id, chunk_index_start, chunk_index_end) VALUES (?1, ?2, ?3)",
        )?;

        chunks
            .iter()
            .map(|(chunk_index_start, chunk_index_end)| {
                stmt.insert(params![page_id, *chunk_index_start, *chunk_index_end])
            })
            .collect::<Result<Vec<i64>, rusqlite::Error>>()?
    };

    Ok(ids)
}
