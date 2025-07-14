use std::io::{BufRead, BufReader};

use crate::{binance::websocket::stream_to_channel, fs::read};

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

fn read_klines(f: &str) -> Option<Vec<f64>> {
    let file = std::fs::File::open(f).ok()?;
    let reader = BufReader::new(file);
    let mut prices = Vec::new();
    for line in reader.lines().map_while(Result::ok) {
        let entries: Vec<&str> = line.split(",").collect();
        if let Some(entry) = crate::fs::parse::checked_string_to_f64(entries[1].to_string()) {
            prices.push(entry);
        }
    }
    Some(prices)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file1 = "resources/BTCUSDT_klines.txt";
    let file2 = "resources/ETHUSDT_klines.txt";

    let data1 = read_klines(file1).unwrap();
    let data2 = read_klines(file2).unwrap();

    let residuals = math::compute_residuals(&data1, &data2).unwrap();
    for r in residuals {
        println!("{r:?}");
    }

    //let res = math::AugmentedDicketFuller::statistic(&residuals, 1);
    //println!("{res:?}");

    /*
    //binance::historical::get_historical_data("monthly", "ETHUSDT", "1m").await?;

    let (tx1, rx1) = std::sync::mpsc::channel::<models::KlineEvent>();
    let (tx2, rx2) = std::sync::mpsc::channel::<models::KlineEvent>();
    let interval = binance::websocket::KlineInterval::OneMinute;
    let btc_handle = stream_to_channel("btcusdt", &interval, tx1);
    let eth_handle = stream_to_channel("ethusdt", &interval, tx2);

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
