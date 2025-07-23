mod binance;
mod errors;
mod fs;
mod math;
mod models;
mod strategy;

use crate::errors::Error;
use crate::fs::read::read_csv_from_zip_file;
use crate::models::{FromDelimitedString, HistoricalKlineEvent};
use crate::strategy::decision::{PositionAction, PositionDirection, PositionParameters};
use crate::strategy::simple::{SimpleAverage, SimpleStrategy};
use crate::{binance::stream::stream_to_channel, fs::read::identify_files};
use chrono::{Months, NaiveDate};
use strategy::decision::{HandleStreamEvent, TradingStrategy};

// Identify two (or more) assets that are historically correlated or cointegrated.
// TODO: Ex. btcusdt and ethusdt.
// When their price spread deviates significantly from the typical range,
// bet on the spread reverting back.
// 1. Calculating the spread: e.g., difference or ratio of prices.
// 2. Computing z-score or another standardized measure of spread deviation.
// 3. Defining entry/exit thresholds on the z-score.
// 4. Taking offsetting long/short positions accordingly.

// TODO: Use strategy to identify opportunities, but place orders at 99.99% of the best bid.
// This should allow orders to get filled quickly, while also enjoyging the maker discount
// offered by binance.

// TODO: Monitor order book depth
// Don not place large orders into thin books.
// Break orders into chunks if depth is thin.

// TODO: Use smart order placement
// POST_ONLY
// IOC (Immediate or Cancel)
// FOK (Fill or Kill)

// Test different liquidity horizons.
// How much data should be used to compute z-score ?
// Try other distributions ?

// Just tempporary storing the stream functionality until I've better refactored
// this code.
fn temp_stream() -> Result<(), Error> {
    let (tx1, rx1) = std::sync::mpsc::channel::<models::KlineEvent>();

    let frequency = binance::stream::KlineInterval::OneSecond;
    let btc_handle = stream_to_channel("btcusdt", &frequency, tx1);

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

fn generate_date_range(start: NaiveDate, end: NaiveDate) -> impl Iterator<Item = NaiveDate> {
    let mut current = start;
    std::iter::from_fn(move || {
        if current <= end {
            if let Some(next) = current.checked_add_months(Months::new(1)) {
                let previous = current;
                current = next;
                return Some(previous);
            }
        }
        None
    })
}

#[derive(Debug)]
struct TrackPositionMovement {
    price: f64,
    total_value: f64,
}

// As a percentage, i.e. 1 = 1%
// 25% discount when using BNB to pay fees.
const FEES: f64 = 0.1 * 0.75;

impl Default for TrackPositionMovement {
    fn default() -> Self {
        Self {
            price: 0.0,
            total_value: 1000.0,
        }
    }
}

impl TrackPositionMovement {
    fn buy(&mut self, price: &f64) {
        self.price = price.to_owned();
        self.total_value *= 1.0 - (FEES / 100.0);
    }

    fn sell(&mut self, price: &f64, position: &Option<PositionDirection>) {
        let profit = match position {
            Some(PositionDirection::Long) => (price - self.price) / price,
            Some(PositionDirection::Short) => (self.price - price) / price,
            None => return,
        };

        self.total_value = self.total_value * (1.0 + profit);
        self.total_value *= 1.0 - (FEES / 100.0);
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Retrieve data.

    let start = NaiveDate::from_ymd_opt(2019, 7, 1).unwrap();
    // let start = NaiveDate::from_ymd_opt(2024, 8, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(2025, 6, 1).unwrap();
    let symbol = "ETHUSDT";
    let frequency = "monthly";
    let interval = "1h";

    let date_range = generate_date_range(start, end);

    binance::historical::retrieve_and_save_historical_data_range(
        date_range, frequency, symbol, interval,
    )
    .await?;

    let base_dir = format!("data/spot/{frequency}/klines/{symbol}/{interval}");

    // TODO: Don't simply load all files in the directory.
    // Rather allow the user to specify the range? Maybe
    // create one struct that allows for the files
    // to be both downloaded and loaded?

    let file_collection = identify_files(base_dir)?;
    let mut prices = Vec::new();
    for file_dir in file_collection {
        let content = read_csv_from_zip_file(file_dir.as_path()).await?;
        for line in content.lines() {
            let entries = HistoricalKlineEvent::from_delimited_string(&line, ',')?;
            prices.push(entries);
        }
    }

    prices.sort();

    let mut position = PositionParameters::default();
    let mut strategy = SimpleAverage::default();
    let mut book = TrackPositionMovement::default();

    for trade in prices {
        strategy.handle_stream_event(&trade.clone().into())?;
        position.set_action(strategy.determine_action(position.direction()));

        match position.action() {
            Some(PositionAction::Buy(_)) => book.buy(&trade.c),
            Some(PositionAction::Sell) => book.sell(&trade.c, &position.direction()),
            _ => continue,
        }

        position
            .set_direction(strategy.determine_direction(position.direction(), position.action()));

        // println!(
        //     "{:<15} {:<15} {:<15}",
        //     format!("{:?}", &position.action()),
        //     format!("{:?}", &position.direction()),
        //     format!("{:?}", &book.total_value),
        // );
    }
    println!("{:?}", book.total_value);
    Ok(())
}
