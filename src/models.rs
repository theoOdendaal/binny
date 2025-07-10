use serde::de;
use serde::{Deserialize, Deserializer};

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'a>,
{
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Debug, Deserialize)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

// btcusdt@depth5@100ms
// <symbol>@depth<levels> OR <symbol>@depth<levels>@100ms
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: usize,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
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
