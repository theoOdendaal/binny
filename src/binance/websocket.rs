use std::sync::mpsc::Sender;
use std::thread;

use tungstenite::{Message, connect};
use url::Url;

use crate::errors;
use crate::models::RawTradeInformation;

static BASE_END_POINT: &str = "wss://stream.binance.com:9443";

pub fn stream_to_channel(
    symbol: &str,
    sender: Sender<RawTradeInformation>,
) -> thread::JoinHandle<Result<(), errors::Error>> {
    let stream = format!("{symbol}@trade");
    let end_point = format!("{BASE_END_POINT}/ws/{stream}");
    let url = Url::parse(&end_point).expect("Invalid URL");

    thread::spawn(move || {
        let (mut socket, _) = connect(url)?;

        while let Ok(msg) = socket.read_message() {
            if let Message::Text(text) = msg {
                let parsed: RawTradeInformation = serde_json::from_str(&text)?;
                if sender.send(parsed).is_err() {
                    break;
                }
            }
        }

        Ok(())
    })
}
/*
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //binance::historical::get_historical_data("monthly", "ETHUSDT", "1m").await?;

    let (tx, rx) = std::sync::mpsc::channel::<RawTradeInformation>();
    let handle = stream_to_channel("ethbtc", tx);

    for trade in rx {
        println!("Received trade: {trade:?}");
    }

    if let Err(e) = handle.join().unwrap() {
        eprintln!("WebSocket thread error: {e}");
    }

    Ok(())
}
*/
