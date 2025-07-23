mod binance;
mod errors;
mod fs;
mod math;
mod models;
mod strategy;

use crate::binance::stream::stream_to_channel;
use crate::errors::Error;
use crate::strategy::decision::PositionParameters;
use crate::strategy::simple::SimpleAverage;
use strategy::decision::{HandleStreamEvent, TradingStrategy};

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

    let interval = binance::stream::KlineInterval::OneSecond;
    let btc_handle = stream_to_channel("btcusdt", &interval, tx1);

    let handle1 = std::thread::spawn(move || -> Result<(), Error> {
        let mut position = PositionParameters::default();
        let mut strategy = SimpleAverage::default();

        for trade in rx1 {
            strategy.handle_stream_event(&trade)?;
            position.set_action(strategy.determine_action(position.direction()));
            position.set_direction(
                strategy.determine_direction(position.direction(), position.action()),
            );

            println!(
                "{:<15} {:<15} {:<15}",
                format!("{:?}", &position.action()),
                format!("{:?}", &position.direction()),
                format!("{:?}", &trade.k.c)
            );
        }
        Ok(())
    });

    btc_handle.join().unwrap()?;
    handle1.join().unwrap()?;
    Ok(())
}
