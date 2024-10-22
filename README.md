# Semantic Search CLI for PDF Files

This Rust-based command-line tool allows you to index PDF files and perform semantic searches on their content. It uses advanced natural language processing techniques to understand the meaning behind your search queries and find relevant results within the indexed PDFs.

[![asciicast](https://asciinema.org/a/XAMRn9IOvU7lN5GDcUehJJYkX.svg)](https://asciinema.org/a/XAMRn9IOvU7lN5GDcUehJJYkX)

## ⚠️ Work in Progress

Please note that this CLI is still under active development and is not yet available as a pre-built binary. Users will need to build and run the project using Cargo commands.

## Features

- Index multiple PDF files for fast searching
- Perform semantic searches on indexed content
- Extract text from both PDF pages and embedded images
- Cache index data for improved performance

### Building and Running

To use this CLI in its current state:

1. Ensure you have Rust and Cargo installed on your system.
2. Install tesseract package for parsing PDF
   ```
   brew install tesseract
   ```
3. Clone this repository:
   ```
   git clone https://github.com/breakpointninja/semantic_search_cli.git
   cd semantic_search_cli
   ```
4. To build the project:
   ```
   cargo build
   ```
5. To run the CLI:
   ```
   cargo run -- [arguments]
   ```

## Usage

The CLI offers two main commands: `index` and `search`.

### Indexing PDF Files

To index PDF files, use the following command:

```shell
semantic_search_cli index <FILES>...
```

Replace `<FILES>...` with the paths to the PDF files you want to index.

### Searching Indexed PDFs

To search the indexed PDF files, use the following command:

```shell
semantic_search_cli search <QUERY>
```

Replace `<QUERY>` with your search query.

## Technical Details

- The tool caches the index in the user's local data directory for faster subsequent searches.
- It runs on the amazing [ort](https://ort.pyke.io/) runtime for fast vector embedding generation.
- It uses the [usearch](https://github.com/unum-cloud/usearch) crate to perform efficient semantic search operations.
- Text extraction from PDF pages is handled by the [pdfium-render](https://github.com/ajrcarey/pdfium-renders) crate.
- The [image](https://github.com/image-rs/image) crate is used to extract text from images embedded in PDFs.
- Search results and PDF file details are stored using SQLite via the [rusqlite](https://github.com/rusqlite/rusqlite) crate.
- Embeddings for PDF content are generated using the [fastembed](https://github.com/qdrant/fastembed) crate.
- The [BAAI/bge-base-en-v1.5](https://huggingface.co/BAAI/bge-base-en-v1.5) embedding model is used to generate embeddings for search queries.
- Chunking is implemented using a naive, brute-force approach with windowed embedding of overlapping chunks.
- Search results are sorted by their distance from the query embedding.

## Limitations and Future Improvements

- This tool is a first draft and is not optimized or production-ready.
- Indexing is currently single-threaded due to thread safety limitations in the `fastembed` and `pdfium-render` crates.
- The chunking algorithm is basic and could be improved for better performance and accuracy.

