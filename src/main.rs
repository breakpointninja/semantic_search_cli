mod chunk_text;
mod database;
mod generate_embeddings;
mod image_to_text;
mod index;
mod index_pdf;
mod lazy_init;
mod pdf_to_image;
mod pdf_to_text;
mod search_index;
mod vector_index;

use crate::index::{index_files, search_with_query};
use clap::{Parser, Subcommand};
use std::env;

#[derive(Parser)]
#[clap(name = "semantic_search_cli")]
#[clap(about = "A semantic search tool for PDF files")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Index PDF files for searching
    Index {
        /// List of PDF files to index
        #[clap(required = true)]
        files: Vec<String>,
    },
    /// Search indexed PDF files
    Search {
        /// Search query
        #[clap(required = true)]
        query: String,
    },
}

fn main() {
    if env::var("RUST_LOG").is_err() {
        // Set default log level if RUST_LOG is not set
        env::set_var("RUST_LOG", "semantic_search_cli=info")
    }

    env_logger::init();

    log::debug!("Starting");

    let cli = Cli::parse();

    match &cli.command {
        Commands::Index { files } => {
            log::debug!("Indexing ...");
            index_files(&files).unwrap();
        }
        Commands::Search { query } => {
            log::debug!("Searching ...");
            search_with_query(&query).unwrap();
        }
    }
}
