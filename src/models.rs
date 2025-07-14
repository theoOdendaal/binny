use serde::de;
use serde::{Deserialize, Deserializer};

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

// btcusdt@trade
// <symbol>@trade
#[derive(Debug, Deserialize)]
pub struct RawTradeInformation {
    pub e: String, // Event type
    pub E: u64,    // Event time (timestamp in ms)
    pub s: String, // Symbol
    pub t: u64,    // Trade ID
    #[serde(deserialize_with = "de_float_from_str")]
    pub p: f32, // Price (as string, to preserve precision)
    #[serde(deserialize_with = "de_float_from_str")]
    pub q: f32, // Quantity (as string)
    pub T: u64,    // Trade time (timestamp in ms)
    pub m: bool,   // Is buyer the market maker?
    pub M: bool,   // Ignore
}

#[derive(Debug, Deserialize)]
pub struct ExchangeInfo {
    pub symbols: Vec<SymbolInfo>,
}

#[derive(Debug, Deserialize)]
pub struct SymbolInfo {
    pub symbol: String,
    pub status: String,
    pub baseAsset: String,
    pub quoteAsset: String,
}

#[derive(Debug, Deserialize)]
pub struct KlineEvent {
    e: String, //Event type
    E: u64,    //Event time
    s: String, //Symbol
    k: Kline,
}

#[derive(Debug, Deserialize)]
pub struct Kline {
    pub t: u64,    // Kline start time
    pub T: u64,    // Kline close time
    pub s: String, // Symbol
    pub i: String, // Interval (e.g. "1m")
    pub f: u64,    // First trade ID
    pub L: u64,    // Last trade ID
    pub o: String, // Open price
    pub c: String, // Close price
    pub h: String, // High price
    pub l: String, // Low price
    pub v: String, // Volume (base asset)
    pub n: u64,    // Number of trades
    pub x: bool,   // Is this kline closed?
    pub q: String, // Quote asset volume
    pub V: String, // Taker buy base asset volume
    pub Q: String, // Taker buy quote asset volume
    pub B: String, // Unused, can be ignored
}
