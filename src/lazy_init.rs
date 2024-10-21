use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};
use lazy_static::lazy_static;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub const EMBEDDING_MODEL: EmbeddingModel = EmbeddingModel::BGEBaseENV15;
const DATA_DIR_NAME: &str = "ninja.breakpoint.semantic_search_cli";

lazy_static! {
    pub static ref DATA_DIR: PathBuf = {
        let data_dir = dirs::data_local_dir()
            .expect("Unable to find local data dir")
            .join(DATA_DIR_NAME);
        create_dir_all(data_dir.clone()).unwrap();
        data_dir
    };
    pub static ref TEXT_EMBEDDING: TextEmbedding = {
        let options = InitOptions::new(EMBEDDING_MODEL)
            .with_cache_dir(DATA_DIR.clone())
            .with_show_download_progress(true);

        TextEmbedding::try_new(options).unwrap()
    };
}
