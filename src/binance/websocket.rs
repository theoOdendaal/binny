use std::sync::mpsc::Sender;
use std::thread;

use serde::{Deserialize, Serialize};
use tungstenite::{Message, connect};
use url::Url;

use crate::{errors, models};

static BASE_END_POINT: &str = "wss://stream.binance.com:9443";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum KlineInterval {
    #[serde(rename = "1s")]
    OneSecond,
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
}

impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::OneSecond => "1s",
            Self::OneMinute => "1m",
            Self::ThreeMinutes => "3m",
            Self::FiveMinutes => "5m",
            Self::FifteenMinutes => "15m",
            Self::ThirtyMinutes => "30m",
            Self::OneHour => "1h",
            Self::TwoHours => "2h",
            Self::FourHours => "4h",
            Self::SixHours => "6h",
            Self::EightHours => "8h",
            Self::TwelveHours => "12h",
            Self::OneDay => "1d",
            Self::ThreeDays => "3d",
            Self::OneWeek => "1w",
            Self::OneMonth => "1M",
        };
        write!(f, "{s}")
    }
}

// TODO: make this more abstract in order to allow streams
// other than Klines to be received.

// TODO: make the below fn more modular because
// the trim_matches() should not be in the below.

pub fn stream_to_channel(
    symbol: &str,
    interval: &KlineInterval,
    sender: Sender<models::KlineEvent>,
) -> thread::JoinHandle<Result<(), errors::Error>> {
    //let stream = format!("{symbol}@trade");
    let stream = format!("{symbol}@kline_{interval}");
    let end_point = format!("{BASE_END_POINT}/ws/{stream}");
    let url = Url::parse(&end_point).expect("Invalid URL");

    thread::spawn(move || {
        let (mut socket, _) = connect(url)?;

        while let Ok(msg) = socket.read_message() {
            if let Message::Text(text) = msg {
                println!("{:?}", text);
                let text = text.trim_matches('"');
                let parsed: models::KlineEvent = serde_json::from_str(&text)?;
                if sender.send(parsed).is_err() {
                    break;
                }
            }
        }

        Ok(())
    })
}
