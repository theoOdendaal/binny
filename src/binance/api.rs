use crate::models;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::BufWriter};
const BASE: &str = "https://api.binance.com/api/v3";

async fn get_klines(symbol: &str) -> Result<Vec<Vec<String>>, Box<dyn std::error::Error>> {
    let url = format!("{BASE}/klines?symbol={symbol}&interval=1d&limit=1000");

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
    let file = File::create(format!("resources/{symbol}_klines.txt")).await?;
    let mut writer = BufWriter::new(file);

    for row in response.iter() {
        let str_vec: Vec<String> = row.iter().map(|s| s.to_string()).collect();
        writer
            .write_all(format!("{}\n", str_vec.join(",")).as_bytes())
            .await?;
    }

    Ok(())
}

async fn get_exchange_information() -> Result<models::ExchangeInfo, Box<dyn std::error::Error>> {
    let url = "{BASE}/exchangeInfo";
    let response = reqwest::get(url)
        .await?
        .json::<models::ExchangeInfo>()
        .await?;
    Ok(response)
}

async fn export_exchange_information() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::create("resources/binance_symbols.txt").await?;
    let mut writer = BufWriter::new(file);

    let response = get_exchange_information().await?;

    for s in response
        .symbols
        .iter()
        .filter(|sym| sym.status == "TRADING" && sym.quoteAsset == "USDT")
    {
        writer
            .write_all(
                format!(
                    "{},{},{},{}\n",
                    s.symbol, s.status, s.baseAsset, s.quoteAsset
                )
                .as_bytes(),
            )
            .await?;
    }

    Ok(())
}
