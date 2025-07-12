use ndarray::Array1;
use serde::de::value;
use std::collections::{self, HashMap, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use tungstenite::connect;
use url::Url;

use crate::math::interpret_adf;

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
/*
static BASE_END_POINT: &str = "wss://stream.binance.com:9443";
fn main() {
    //let stream = "btcusdt@depth5@100ms";
    //let stream = "btcusdt@trade";
    let stream = "ethusdt@trade";
    let end_point = format!("{BASE_END_POINT}/ws/{stream}");

    let url = Url::parse(&end_point).unwrap();
    let (mut socket, response) = connect(url).expect("Can't connect.");

    println!("Connected to binance stream.");
    println!("HTTP status code: {}", response.status());
    println!("Response headers:");
    for (ref header, ref header_value) in response.headers() {
        println!("- {header}: {header_value:?}");
    }
    println!();
    let mut prices = VecDeque::with_capacity(100);
    let mut quantities = VecDeque::with_capacity(100);

    loop {
        let msg = socket.read_message().expect("Error reading message");

        if let tungstenite::Message::Text(s) = msg {
            let parsed: models::RawTradeInformation =
                serde_json::from_str(&s).expect("Can't parse");

            let symbol = parsed.s;
            let price = parsed.p;
            let quantity = parsed.q;
            print!("{symbol} {price} {quantity} ");

            if prices.len() == 100 {
                prices.pop_front();
                quantities.pop_front();
            }
            prices.push_back(price);
            quantities.push_back(quantity);

            let average = prices.iter().sum::<f64>() / (prices.len() as f64);

            print!("{average}");
            println!();
        }
    }
}
*/

fn parse_string_to_f64(s: String) -> Option<f64> {
    let trimmed = s.as_str().trim_matches('"');
    trimmed.parse::<f64>().ok()
}

fn read_symbols_from_file(path: &str) -> std::io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let symbols = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(symbols)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Extract klines for a collection of symbols.
    // Used to identify correlated pairs.
    //let symbols = read_symbols_from_file("resources/binance_symbols.txt")?;
    //let parsed_symbols: Vec<&str> = symbols.iter().map(|s| s.as_str()).collect();
    //export_pearson_correlations(&parsed_symbols).await?;

    let symbols = ["BTCUSDT", "ETHUSDT"];
    let x: Array1<f64> = get_klines(symbols[0])
        .await?
        .iter()
        .filter_map(|p| parse_string_to_f64(p[1].clone()))
        .collect();

    let y: Array1<f64> = get_klines(symbols[1])
        .await?
        .iter()
        .filter_map(|p| parse_string_to_f64(p[1].clone()))
        .collect();

    println!("{:?}", x);

    /*
    let x_statistic = math::augmented_dickey_fuller_statistic(&x, 2);
    let y_statistic = math::augmented_dickey_fuller_statistic(&y, 2);

    println!("{:?}", interpret_adf(x_statistic));
    println!("{:?}", interpret_adf(y_statistic));
    */
    Ok(())
}

async fn get_pearson_correlations<'a>(
    symbols: &'a [&str],
) -> Result<HashMap<(&'a str, &'a str), f64>, Box<dyn std::error::Error>> {
    let mut observations: Vec<Vec<f64>> = Vec::new();

    for s in symbols {
        let values = get_klines(s).await?;

        let prices: Vec<f64> = values
            .iter()
            .filter_map(|p| parse_string_to_f64(p[1].clone()))
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
