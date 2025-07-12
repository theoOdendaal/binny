use chrono::{Months, TimeZone};

use crate::binance::websocket::stream_to_channel;
use crate::fs::read::read_zip_file;
use crate::models::RawTradeInformation;

mod binance;
mod errors;
mod fs;
mod math;
mod models;

// Identify two (or more) assets that are historically correlated or cointegrated.
// Ex. btcusdt and ethusdt.
// When their price spread deviates significantly from the typical range,
// bet on the spread reverting back.
// 1. Calculating the spread: e.g., difference or ratio of prices.
// 2. Computing z-score or another standardized measure of spread deviation.
// 3. Defining entry/exit thresholds on the z-score.
// 4. Taking offsetting long/short positions accordingly.

// Test different liquidity horizons.
// How much data should be used to compute z-score ?
// Try other distributions ?

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //binance::historical::get_historical_data("monthly", "ETHUSDT", "1m").await?;

    let start_date = chrono::NaiveDate::from_ymd_opt(2017, 8, 1).unwrap();
    let end_data = chrono::NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    let mut data = Vec::new();

    let mut current_date = start_date;
    while current_date <= end_data {
        let file = &format!(
            "raw_data/BTCUSDT/BTCUSDT-1m-{}.zip",
            current_date.format("%Y-%m")
        );
        println!("{:?}", &file);
        let content = read_zip_file(file).await?;

        for line in content.lines() {
            let entries: Vec<&str> = line.split(",").take(2).collect();
            if let (Some(ts_str), Some(price)) = (entries.first(), entries.get(1)) {
                if let Ok(ms) = ts_str.parse::<i64>() {
                    let secs = ms / 1000;
                    let nsecs = ((ms % 1000) * 1_000_000) as u32;

                    if let chrono::LocalResult::Single(datetime) =
                        chrono::Utc.timestamp_opt(secs, nsecs)
                    {
                        data.push((datetime, price.to_string()));
                    }
                }
            }
        }
        current_date = current_date.checked_add_months(Months::new(1)).unwrap();
    }

    println!("{data:?}");

    /*
    let (tx1, rx1) = std::sync::mpsc::channel::<RawTradeInformation>();
    let (tx2, rx2) = std::sync::mpsc::channel::<RawTradeInformation>();
    let btc_handle = stream_to_channel("btcusdt", tx1);
    let eth_handle = stream_to_channel("ethusdt", tx2);

    let handle1 = std::thread::spawn(move || {
        for trade in rx1 {
            println!("[ETH] {trade:?}");
        }
    });

    let handle2 = std::thread::spawn(move || {
        for trade in rx2 {
            println!("[BTC] {trade:?}");
        }
    });

    // Wait for all threads
    btc_handle.join().unwrap()?;
    eth_handle.join().unwrap()?;
    handle1.join().unwrap();
    handle2.join().unwrap();
    */
    Ok(())
}
/*
async fn get_pearson_correlations<'a>(
    symbols: &'a [&str],
) -> Result<HashMap<(&'a str, &'a str), f64>, Box<dyn std::error::Error>> {
    let mut observations: Vec<Vec<f64>> = Vec::new();

    for s in symbols {
        let values = get_klines(s).await?;

        let prices: Vec<f64> = values
            .iter()
            .filter_map(|p| fs::parse::checked_string_to_f64(p[1].clone()))
            .collect();

        let returns = math::to_log_returns(&prices);

        observations.push(returns);
    }
    let mut correlations = HashMap::new();
    for (a1, o1) in symbols.iter().zip(observations.iter()) {
        for (a2, o2) in symbols.iter().zip(observations.iter()) {
            //if a1 != a2 {
            //    let reverse_key = (*a2, *a1);
            //    if !correlations.contains_key(&reverse_key) {
            println!("{a1} {a2}");
            correlations.insert((*a1, *a2), math::pearson_correlation(o1, o2));
            //    }
            //}
        }
    }
    Ok(correlations)
}

async fn export_pearson_correlations(symbols: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let responses = get_pearson_correlations(symbols).await?;

    let file = File::create("resources/correlations.txt")?;
    let mut writer = BufWriter::new(file);
    for ((ka, kb), v) in responses.iter() {
        writeln!(writer, "{ka},{kb},{v}")?;
    }
    Ok(())
}

async fn get_klines(symbol: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let url =
        format!("https://api.binance.com/api/v3/klines?symbol={symbol}&interval=1d&limit=1000");

    let response = reqwest::get(url)
        .await?
        .json::<Vec<Vec<serde_json::Value>>>()
        .await?;
    println!("GET {symbol}");
    Ok(response
        .iter()
        .map(|row| row.iter().map(|entry| entry.to_string()).collect())
        .collect())
}

async fn export_klines(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get_klines(symbol).await?;
    let file = File::create(format!("resources/{symbol}_klines.txt"))?;
    let mut writer = BufWriter::new(file);

    for row in response.iter() {
        let str_vec: Vec<String> = row.iter().map(|s| s.to_string()).collect();
        writeln!(writer, "{}", str_vec.join(","))?;
    }

    Ok(())
}

async fn get_exchange_information() -> Result<models::ExchangeInfo, Box<dyn std::error::Error>> {
    let url = "https://api.binance.com/api/v3/exchangeInfo";
    let response = reqwest::get(url)
        .await?
        .json::<models::ExchangeInfo>()
        .await?;
    Ok(response)
}

async fn export_exchange_information() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("resources/binance_symbols.txt")?;
    let mut writer = BufWriter::new(file);

    let response = get_exchange_information().await?;

    for s in response
        .symbols
        .iter()
        .filter(|sym| sym.status == "TRADING" && sym.quoteAsset == "USDT")
    {
        writeln!(
            writer,
            "{},{},{},{}",
            s.symbol, s.status, s.baseAsset, s.quoteAsset
        )?;
    }

    Ok(())
}
*/
