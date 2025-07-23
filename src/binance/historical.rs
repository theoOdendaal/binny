use chrono::NaiveDate;
use std::path::Path;

use crate::errors::Error;
use crate::fs::write::async_write_safely;

const BASE: &str = "https://data.binance.vision/data";

fn get_remove_file_name(symbol: &str, interval: &str, date: &NaiveDate) -> String {
    format!("{symbol}-{interval}-{}.zip", date.format("%Y-%m"))
}

fn get_remote_file_path(frequency: &str, symbol: &str, interval: &str) -> String {
    format!("{BASE}/spot/{frequency}/klines/{symbol}/{interval}")
}

fn get_local_file_name(symbol: &str, interval: &str, date: &NaiveDate) -> String {
    get_remove_file_name(symbol, interval, date)
}

fn get_local_file_path(frequency: &str, symbol: &str, interval: &str) -> String {
    format!("data/spot/{frequency}/klines/{symbol}/{interval}")
}

fn is_saved(frequency: &str, symbol: &str, interval: &str, date: &NaiveDate) -> bool {
    let local_file_path = get_local_file_path(frequency, symbol, interval);
    let local_file_name = get_local_file_name(symbol, interval, date);
    let local_full_path = Path::new(&local_file_path).join(&local_file_name);
    local_full_path.exists() && local_full_path.is_file()
}

async fn retrieve_historical_data(
    client: &reqwest::Client,
    frequency: &str,
    symbol: &str,
    interval: &str,
    date: &NaiveDate,
) -> Result<Vec<u8>, Error> {
    let path = get_remote_file_path(frequency, symbol, interval);
    let file = get_remove_file_name(symbol, interval, date);
    let url = format!("{}/{}", path, file);
    let response = client.get(url).send().await?;
    let text = response.bytes().await?.to_vec();
    Ok(text)
}

pub async fn retrieve_and_save_historical_data_range<I>(
    dates: I,
    frequency: &str,
    symbol: &str,
    interval: &str,
) -> Result<(), Error>
where
    I: IntoIterator<Item = NaiveDate>,
{
    let client = reqwest::Client::new();
    for date in dates {
        // Data is only downloaded if not yet available.
        if !is_saved(frequency, symbol, interval, &date) {
            let local_path = get_local_file_path(frequency, symbol, interval);
            let local_file = get_local_file_name(symbol, interval, &date);
            let path = Path::new(&local_path).join(&local_file);
            let file =
                retrieve_historical_data(&client, frequency, symbol, interval, &date).await?;
            async_write_safely(path, &file).await?;
            println!("{}", &local_file);
        }
    }
    Ok(())
}
