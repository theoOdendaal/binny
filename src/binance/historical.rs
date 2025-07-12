use std::io::Write;

use crate::errors;
use tokio::io::AsyncWriteExt;

const BASE: &str = "https://data.binance.vision/data";

/*
const INTERVAL: [&str; 2] = ["daily", "monthly"];
const SPOT: [&str; 3] = ["aggTrades", "klines", "trades"];
const FREQUENCY: [&str; 16] = [
    "12h", "15m", "1d", "1h", "1m", "1mo", "1s", "1w", "2h", "30m", "3d", "3m", "4h", "5m", "6h",
    "8h",
];
*/

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

pub async fn get_historical_data(
    interval: &str,
    symbol: &str,
    frequency: &str,
) -> Result<(), errors::Error> {
    let base_url = format!("{BASE}/spot/{interval}/klines/{symbol}/{frequency}");
    let start_date = chrono::NaiveDate::from_ymd_opt(2017, 8, 1).unwrap();
    let end_date = chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    let client = reqwest::Client::new();

    let mut date = start_date;
    while date <= end_date {
        let filename = format!("{symbol}-1m-{}.zip", date.format("%Y-%m"));
        let url = format!("{base_url}/{filename}");
        println!("Downloading: {url}");

        let mut writer = tokio::fs::File::create(format!("raw_data/{symbol}/{filename}")).await?;
        retrieve_file(&client, &mut writer, &url).await?;

        date = date.checked_add_months(chrono::Months::new(1)).unwrap();
    }

    Ok(())
}
