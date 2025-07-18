use std::{fs::create_dir_all, path::Path};

use crate::errors;
use tokio::io::AsyncWriteExt;

const BASE: &str = "https://data.binance.vision/data";

async fn retrieve_file(
    client: &reqwest::Client,
    writer: &mut tokio::fs::File,
    url: &str,
) -> Result<(), errors::Error> {
    let response = client.get(url).send().await?;

    if !response.status().is_success() {
        return Err(errors::Error::Other(format!(
            "Failed to download: {} (status {})",
            url,
            response.status()
        )));
    }

    let bytes = response.bytes().await?;
    writer.write_all(&bytes).await?;

    Ok(())
}

pub async fn get_historical_data_range(
    interval: &str,
    symbol: &str,
    frequency: &str,
) -> Result<(), errors::Error> {
    let dir = format!("spot/{interval}/klines/{symbol}/{frequency}");
    let base_url = format!("{BASE}/{dir}");
    let start_date = chrono::NaiveDate::from_ymd_opt(2017, 8, 1).unwrap();
    let end_date = chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    let client = reqwest::Client::new();

    let mut date = start_date;
    while date <= end_date {
        let filename = format!("{symbol}-{frequency}-{}.zip", date.format("%Y-%m"));
        let url = format!("{base_url}/{filename}");

        let path_dir = &format!("data/{dir}");
        let path = Path::new(path_dir);
        if !path.exists() {
            create_dir_all(path)?;
        }

        // Only downloads new file if not yet available.
        let file_dir = format!("{path_dir}/{filename}");
        let file_path = Path::new(&file_dir);
        if !file_path.exists() {
            println!("Downloading: {url}");
            let mut writer = tokio::fs::File::create(file_path).await?;
            retrieve_file(&client, &mut writer, &url).await?;
        } else {
            println!("Already available: {url}");
        }

        date = date.checked_add_months(chrono::Months::new(1)).unwrap();
    }

    Ok(())
}
