use flate2::bufread::GzDecoder;
use serde::Deserialize;
use std::fs::File;
use std::{
    env,
    io::{self, BufReader},
    path::PathBuf,
};
use tar::Archive;

/// Statically link pdfium-lib to the binary.
/// This is done by downloading the latest release of pdfium-lib for macOS and
/// extracting the libpdfium.a file from the archive.
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    extract_pdfium(&out_dir).unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=pdfium");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
}

#[derive(Deserialize)]
struct Asset {
    name: String,
    browser_download_url: String,
}

#[derive(Deserialize)]
struct Releases {
    assets: Vec<Asset>,
}

/// Download the latest release of pdfium-lib for macOS and extract the libpdfium.a file.
fn extract_pdfium(out_dir: &str) -> anyhow::Result<()> {
    let path: PathBuf = out_dir.into();
    let lib_path = path.join("libpdfium.a");

    // check if lib_path exists already
    if lib_path.exists() {
        println!("{} already exists, skipping download", &lib_path.display());
        return Ok(());
    }

    println!("Downloading pdfium-lib to {}", &lib_path.display());

    // Get the latest pdfium-lib release
    let releases =
        ureq::get("https://api.github.com/repos/paulocoutinhox/pdfium-lib/releases/latest")
            .call()?
            .body_mut()
            .read_json::<Releases>()?;

    // Find an asset with name "macos.tgz"
    let asset = releases
        .assets
        .into_iter()
        .find(|asset| asset.name == "macos.tgz")
        .ok_or(anyhow::anyhow!("No MacOS asset found"))?;

    // Download the asset
    let tar_reader = ureq::get(&asset.browser_download_url)
        .call()?
        .into_body()
        .into_reader();

    // Parse the archive
    let mut archive = Archive::new(GzDecoder::new(BufReader::new(tar_reader)));

    // Extract the libpdfium.a file from the archive
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        if path.ends_with("libpdfium.a") {
            let mut file = File::create(&lib_path)?;
            io::copy(&mut entry, &mut file)?;

            return Ok(());
        }
    }

    // Failed to find libpdfium.a in the archive
    Err(anyhow::anyhow!("Failed to find libpdfium.a in the archive"))
}
