use crate::database::Database;
use crate::index_pdf::index_pdf;
use crate::search_index::search_index;
use pdfium_render::prelude::*;

use crate::lazy_init::DATA_DIR;
use crate::vector_index;
use crate::vector_index::load_vector_index;
use std::path::PathBuf;

const DB_NAME: &str = "db.sqlite";

pub fn index_files(files: &Vec<String>) -> anyhow::Result<()> {
    log::debug!("Data directory: {}", &DATA_DIR.display());

    // Load sqlite database
    let mut db = get_db(&DATA_DIR)?;
    let index = load_vector_index(&DATA_DIR)?;
    let pdfium = Pdfium::new(Pdfium::bind_to_statically_linked_library()?);

    for file in files.iter() {
        let path = PathBuf::from(file);
        if !path.exists() {
            log::warn!("File {} does not exists", path.display());
            continue;
        }

        let path = path.canonicalize()?;
        if db.document_exists(&path.to_str().unwrap())? {
            log::warn!("File {} is already indexed", path.display());
            continue;
        }

        index_pdf(&pdfium, &path, &mut db, &index)
            .unwrap_or_else(|e| log::error!("Error indexing file {}: {}", path.display(), e));
    }

    vector_index::save_vector_index(&index)?;

    // search_index("Conclusion or Insights of results", &db, &index)?;

    log::debug!("Done");

    Ok(())
}

pub fn search_with_query(query: &str) -> anyhow::Result<()> {
    use colored::*;

    // Load sqlite database
    let db = get_db(&DATA_DIR)?;
    let index = load_vector_index(&DATA_DIR)?;

    for item in search_index(query, &db, &index)? {
        let item = item?;
        println!(
            "{}",
            "=================================================================================="
                .red()
        );

        println!(
            "{}\nDistance: {}\nPage No: {}\n\n{}",
            item.path.green(),
            item.distance.to_string().bright_red(),
            item.page_no.to_string().blue(),
            item.text
        );
    }

    Ok(())
}

fn get_db(data_dir: &PathBuf) -> anyhow::Result<Database> {
    let db_path = data_dir.join(DB_NAME);
    let db = Database::new(&db_path)?;
    db.init_tables()?;

    Ok(db)
}
