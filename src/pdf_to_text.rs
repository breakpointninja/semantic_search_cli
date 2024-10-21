use crate::image_to_text::image_ocr;
use crate::pdf_to_image::PDFImages;
use pdfium_render::prelude::{Pdfium, PdfiumError};
use std::path::Path;

pub struct PDFText<'a> {
    pdf_images: PDFImages<'a>,
}

impl PDFText<'_> {
    pub fn new<'a>(
        pdfium: &'a Pdfium,
        path: &impl AsRef<Path>,
    ) -> Result<PDFText<'a>, PdfiumError> {
        Ok(PDFText {
            pdf_images: PDFImages::new(pdfium, path)?,
        })
    }
}

impl<'a> Iterator for PDFText<'a> {
    type Item = anyhow::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pdf_images.next().map(|image| image_ocr(&image?))
    }
}
