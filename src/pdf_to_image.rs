use image::DynamicImage;
use pdfium_render::prelude::{
    PdfDocument, PdfPageRenderRotation, PdfRenderConfig, Pdfium, PdfiumError,
};
use std::path::Path;

pub struct PDFImages<'a> {
    document: PdfDocument<'a>,
    page: u16,
    render_config: PdfRenderConfig,
}

impl PDFImages<'_> {
    pub fn new<'a>(
        pdfium: &'a Pdfium,
        path: &impl AsRef<Path>,
    ) -> Result<PDFImages<'a>, PdfiumError> {
        Ok(PDFImages {
            document: pdfium.load_pdf_from_file(path, None)?,
            page: 0,
            render_config: PdfRenderConfig::new()
                .set_target_width(2000)
                .set_maximum_height(2000)
                .rotate_if_landscape(PdfPageRenderRotation::Degrees90, true),
        })
    }
}

impl<'a> Iterator for PDFImages<'a> {
    type Item = Result<DynamicImage, PdfiumError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.page < self.document.pages().len() {
            let result = self
                .document
                .pages()
                .get(self.page)
                .and_then(|page| Ok(page.render_with_config(&self.render_config)?.as_image()));

            self.page += 1;
            Some(result)
        } else {
            None
        }
    }
}
