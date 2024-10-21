use crate::generate_embeddings::EMBEDDING_DIMENSIONS;
use crate::lazy_init::DATA_DIR;
use std::path::PathBuf;
use usearch::{Index, IndexOptions, MetricKind, ScalarKind};

const INDEX_NAME: &str = "index.usearch";

pub fn load_vector_index(data_dir: &PathBuf) -> anyhow::Result<Index> {
    let mut options = IndexOptions::default();
    options.dimensions = EMBEDDING_DIMENSIONS; // Set the number of dimensions for vectors
    options.metric = MetricKind::Cos; // Use cosine similarity for distance measurement
    options.quantization = ScalarKind::F32; // Use 32-bit floating point numbers

    let index = Index::new(&options)?;
    let vector_index_path = data_dir.join(INDEX_NAME);
    if vector_index_path.exists() {
        // TODO: Handle non-unicode paths
        index.load(&vector_index_path.to_str().unwrap())?;
    }

    Ok(index)
}

pub fn save_vector_index(index: &Index) -> anyhow::Result<()> {
    let vector_index_path = DATA_DIR.join(INDEX_NAME);
    index.save(&vector_index_path.to_str().unwrap())?;

    Ok(())
}
