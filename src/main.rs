use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufWriter, Write};
use tungstenite::connect;
use url::Url;

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

            let average = prices.iter().sum::<f32>() / (prices.len() as f32);

            print!("{average}");
            println!();
        }
    }
}
*/

fn parse_string_to_f32(s: String) -> Option<f32> {
    let string_ref = s.as_str();
    string_ref.parse::<f32>().ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let btc_usdt: Vec<f32> = get_klines("BTCUSDT")
        .await?
        .iter()
        .filter_map(|row| parse_string_to_f32(row[1].clone()))
        .collect();

    let eth_usdt: Vec<f32> = get_klines("ETHUSDT")
        .await?
        .iter()
        .filter_map(|row| parse_string_to_f32(row[1].clone()))
        .collect();

    let spread_ratio: Vec<f32> = btc_usdt
        .iter()
        .zip(eth_usdt.iter())
        .map(|(a, b)| b / a)
        .collect();

    let spread_average = spread_ratio.iter().sum::<f32>() / (spread_ratio.len() as f32);
    let spread_std_dev: f32 = spread_ratio
        .iter()
        .map(|a| (a - spread_average).powf(2.0))
        .sum();
    let spread_std_dev = spread_std_dev.powf(1.0 / 2.0);

    let spread_z_scores: Vec<f32> = spread_ratio
        .iter()
        .map(|a| (a - spread_average) / spread_std_dev)
        .collect();

    println!("{spread_z_scores:?}");

    Ok(())
}

async fn get_klines(symbol: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let url =
        format!("https://api.binance.com/api/v3/klines?symbol={symbol}&interval=1h&limit=1000");

    let response = reqwest::get(url)
        .await?
        .json::<Vec<Vec<serde_json::Value>>>()
        .await?;

    Ok(response
        .iter()
        .map(|row| row.iter().map(|entry| entry.to_string()).collect())
        .collect())
}

async fn export_klines(symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url =
        format!("https://api.binance.com/api/v3/klines?symbol={symbol}&interval=1h&limit=1000");
    let response = reqwest::get(url)
        .await?
        .json::<Vec<Vec<serde_json::Value>>>()
        .await?;

    let file = File::create(format!("resources/{symbol}_klines.txt"))?;
    let mut writer = BufWriter::new(file);

    for row in response {
        let str_vec: Vec<String> = row.iter().map(|s| s.to_string()).collect();
        writeln!(writer, "{}", str_vec.join(","))?;
    }

    Ok(())
}

/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://api.binance.com/api/v3/exchangeInfo";
    let response = reqwest::get(url)
        .await?
        .json::<models::ExchangeInfo>()
        .await?;

    let file = File::create("resources/binance_symbols.txt")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "symbol,status,baseAsset,quoteAsset")?;

    for s in response.symbols {
        writeln!(
            writer,
            "{},{},{},{}",
            s.symbol, s.status, s.baseAsset, s.quoteAsset
        )?;
    }

    Ok(())
}
*/
