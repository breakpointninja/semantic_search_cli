use crate::chunk_text::sliding_window_chunk_indices;
use crate::database::{insert_chunks, insert_document, insert_page, Database};
use crate::generate_embeddings::{generate_embeddings, EMBEDDING_BATCH_SIZE};
use crate::pdf_to_text::PDFText;
use crate::vector_index::save_vector_index;
use itertools::Itertools;
use pdfium_render::prelude::Pdfium;
use std::path::Path;
use usearch::Index;

const CHUNK_SIZE: usize = 512;
const CHUNK_STRIDE: usize = 64;

pub fn index_pdf(
    pdfium: &Pdfium,
    path: &impl AsRef<Path>,
    db: &mut Database,
    index_db: &Index,
) -> anyhow::Result<()> {
    log::info!("Indexing PDF at path {}", path.as_ref().display());

    let pdf_text = PDFText::new(&pdfium, path)?;

    // Normalize path
    let path = path.as_ref().canonicalize()?;
    // TODO Better handle non-UTF8 paths
    let path = String::from(path.to_str().unwrap());

    let tx = db.conn.transaction()?;
    let doc_id = insert_document(&tx, &path)?;

    for (page_no, text) in pdf_text.enumerate() {
        log::info!("Indexing page {} of {}", page_no, path);
        let text = text?;
        let page_id = insert_page(&tx, doc_id, page_no, &text)?;

        for chunk_of_indices in &sliding_window_chunk_indices(&text, CHUNK_SIZE, CHUNK_STRIDE)
            .chunks(EMBEDDING_BATCH_SIZE)
        {
            let mut text_chunks: Vec<&str> = Vec::with_capacity(EMBEDDING_BATCH_SIZE);
            let mut chunk_indices: Vec<(usize, usize)> = Vec::with_capacity(EMBEDDING_BATCH_SIZE);

            for (start, end) in chunk_of_indices {
                text_chunks.push(&text[start..end]);
                chunk_indices.push((start, end));
            }

            let doc_ids = insert_chunks(&tx, page_id, chunk_indices.as_slice())?;
            let embeddings = generate_embeddings(text_chunks)?;

            log::debug!(
                "Inserting {} embeddings into vector database",
                embeddings.len()
            );

            index_db.reserve(index_db.capacity() + doc_ids.len())?;
            for (doc_id, embedding) in doc_ids.into_iter().zip(embeddings.into_iter()) {
                index_db.add(doc_id.try_into()?, &embedding)?;
            }
        }
    }

    save_vector_index(&index_db)?;
    tx.commit()?;

    Ok(())
}
