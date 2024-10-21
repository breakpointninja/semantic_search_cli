use crate::database::Database;
use crate::generate_embeddings::generate_embeddings;
use usearch::ffi::Matches;
use usearch::Index;

pub struct VectorSearch<'a> {
    db: &'a Database,
    matches: Matches,
    index: usize,
}

pub fn search_index<'a>(
    query: &str,
    db: &'a Database,
    index_db: &Index,
) -> anyhow::Result<VectorSearch<'a>> {
    let embeddings = generate_embeddings(vec![query])?;
    let query_embedding = embeddings.into_iter().next().unwrap();

    Ok(VectorSearch {
        db,
        matches: index_db.search(&query_embedding, 10)?,
        index: 0,
    })
}

pub struct SearchResult {
    pub distance: f32,
    pub path: String,
    pub page_no: usize,
    pub text: String,
}

impl Iterator for VectorSearch<'_> {
    type Item = anyhow::Result<SearchResult>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.matches.keys.len() {
            let key = self.matches.keys[self.index];
            let distance = self.matches.distances[self.index];
            self.index += 1;

            Some(
                self.db
                    .get_document(key)
                    .map(|(path, page_no, text)| SearchResult {
                        distance,
                        path,
                        page_no,
                        text,
                    })
                    .map_err(|e| anyhow::anyhow!(e)),
            )
        } else {
            None
        }
    }
}
