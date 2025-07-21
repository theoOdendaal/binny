mod binance;
mod errors;
mod fs;
mod math;
mod models;
mod strategy;

use crate::binance::websocket::stream_to_channel;
use crate::errors::Error;
use crate::strategy::decision::{PositionAction, PositionDirection};
use crate::strategy::simple::{SimpleAverage, SimpleStrategy};
use strategy::decision::TradingDecision;

// Identify two (or more) assets that are historically correlated or cointegrated.
// TODO: Ex. btcusdt and ethusdt.
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
async fn main() -> Result<(), Error> {
    // Retrieve data.
    // binance::historical::get_historical_data_range("monthly", "ETHUSDT", "1m").await?;
    // binance::historical::get_historical_data_range("monthly", "BTCUSDT", "1m").await?;
    // binance::historical::get_historical_data_range("monthly", "DOGEBTC", "1m").await?;
    //
    // let base_dir = "data/spot/monthly/klines/BTCUSDT/1m";
    // let file_collection = identify_files(base_dir)?;
    //
    // let mut prices = Vec::new();
    // for file_dir in file_collection {
    //     let content = read_csv_from_zip_file(file_dir.as_path()).await?;
    //     for line in content.lines() {
    //         let entries = HistoricalKlineEvent::from_delimited_string(&line, ',')?;
    //         let time = entries.t;
    //         let price = entries.c;
    //         prices.push((time, price));
    //     }
    // }
    //
    // // println!("{:?}", &prices);

    let (tx1, rx1) = std::sync::mpsc::channel::<models::KlineEvent>();
    // let (tx2, rx2) = std::sync::mpsc::channel::<models::KlineEvent>();
    let interval = binance::websocket::KlineInterval::OneSecond;
    let btc_handle = stream_to_channel("btcusdt", &interval, tx1);
    // let eth_handle = stream_to_channel("ethusdt", &interval, tx2);

    let handle1 = std::thread::spawn(move || {
        let mut position: Option<PositionDirection> = None;
        let mut strategy = SimpleAverage::default();
        let mut proposed_decision: Option<PositionAction>;

        for trade in rx1 {
            // println!("[BTC] {trade:?}");
            strategy.update(&trade);
            proposed_decision = strategy.trading_decision(position);
            position = strategy.update_decision(position, proposed_decision);
            println!(
                "{:?}\t\t{:?}\t\t{:?}",
                &proposed_decision, &position, &trade.k.c
            );
        }
    });

    // let handle2 = std::thread::spawn(move || {
    //     for trade in rx2 {
    //         println!("[BTC] {trade:?}");
    //     }
    // });

    // Wait for all threads
    btc_handle.join().unwrap()?;
    // eth_handle.join().unwrap()?;
    handle1.join().unwrap();
    // handle2.join().unwrap();
    Ok(())
}
