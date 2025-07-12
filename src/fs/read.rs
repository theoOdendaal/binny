use std::io::Read;

use tokio::io::AsyncBufReadExt;
use zip::ZipArchive;

use crate::errors;

/// Read file, treating each lines as one String.
pub async fn without_delimiter(path: &str) -> Result<Vec<String>, errors::Error> {
    let file = tokio::fs::File::open(path).await?;
    let reader = tokio::io::BufReader::new(file);

    let mut lines = reader.lines();
    let mut parsed_lines = Vec::new();

    while let Some(line) = lines.next_line().await? {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            parsed_lines.push(trimmed.to_string());
        }
    }

    Ok(parsed_lines)
}

/// Reads the content of a CSV file inside a ZIP archive.
/// The function expects that the ZIP archive contains a single CSV file whose
/// filename matches the name of the ZIP file (excluding the `.zip` extension)
/// with a `.csv` extension.
pub async fn read_zip_file(path: &str) -> Result<String, errors::Error> {
    let zip_path = std::path::Path::new(path);
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
