use std::path::{Path, PathBuf};
use std::{fs, io::Read};
use zip::ZipArchive;

use crate::errors;

/// Reads the content of a CSV file inside a ZIP archive.
/// The function expects that the ZIP archive contains a single CSV file whose
/// filename matches the name of the ZIP file (excluding the `.zip` extension)
/// with a `.csv` extension.
pub async fn read_csv_from_zip_file<P: AsRef<Path>>(path: P) -> Result<String, errors::Error> {
    let zip_path = path.as_ref();
    let file = std::fs::File::open(zip_path)?;
    let reader = std::io::BufReader::new(file);
    let mut archive = ZipArchive::new(reader)?;
    let mut content = String::new();

    if let Some(file_stem_osstr) = zip_path.file_stem() {
        if let Some(file_stem) = file_stem_osstr.to_str() {
            let filename = format!("{file_stem}.csv");
            let mut zip_file = archive.by_name(&filename)?;
            zip_file.read_to_string(&mut content)?;
        }
    }
    Ok(content)
}

/// Creates a collection of all files within a
/// specified directory.
pub fn identify_files<P: AsRef<Path>>(path: P) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            files.push(path);
        }
    }
    Ok(files)
}
