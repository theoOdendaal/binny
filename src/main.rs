mod binance;
mod errors;
mod fs;
mod math;
mod models;

use crate::errors::Error;
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};
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
    let file = File::open(f).ok()?;
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

/// Initialize required directories.
fn init_dirs() -> Result<(), Error> {
    let dir_path = ["data"];
    for path in dir_path {
        let dir = Path::new(path);
        if !dir.exists() && !dir.is_dir() {
            std::fs::create_dir(dir)?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_dirs()?;
    // let file1 = "resources/BTCUSDT_klines.txt";
    // let file2 = "resources/ETHUSDT_klines.txt";

    // let data1 = read_klines(file1).unwrap();
    // let data2 = read_klines(file2).unwrap();
    //
    // let residuals = math::compute_residuals(&data1, &data2).unwrap();
    // for r in residuals {
    //     println!("{r:?}");
    // }

    //let res = math::AugmentedDicketFuller::statistic(&residuals, 1);
    //println!("{res:?}");

    binance::historical::get_historical_data_range("monthly", "ETHUSDT", "1m").await?;
    binance::historical::get_historical_data_range("monthly", "BTCUSDT", "1m").await?;
    /*
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
