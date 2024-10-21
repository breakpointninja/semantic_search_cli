use anyhow;
use image::DynamicImage;
use rusty_tesseract::{Args, Image};

pub fn image_ocr(image: &DynamicImage) -> anyhow::Result<String> {
    log::debug!("Extracting text from PDF Image");

    let img = Image::from_dynamic_image(&image)?;
    let default_args = Args::default();

    Ok(rusty_tesseract::image_to_string(&img, &default_args)?)
}
