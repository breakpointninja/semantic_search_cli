use crate::lazy_init::TEXT_EMBEDDING;
use fastembed::Embedding;

pub const EMBEDDING_BATCH_SIZE: usize = 256;
pub const EMBEDDING_DIMENSIONS: usize = 768;

/// Generates embeddings for a list of strings and returns them as a vector of vectors
/// Each vector represents an embedding for a string
pub fn generate_embeddings(strings: Vec<&str>) -> anyhow::Result<Vec<Embedding>> {
    log::debug!("Generating embeddings for {} strings", strings.len());

    // Generate embeddings with the default batch size, 256
    TEXT_EMBEDDING.embed(
        strings
            .iter()
            .map(|s| {
                format!(
                    "Represent this sentence for searching relevant passages: {}",
                    s
                )
            })
            .collect(),
        Some(EMBEDDING_BATCH_SIZE),
    )
}
