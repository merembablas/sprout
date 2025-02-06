use super::config::{CandleRequest, TradeRequest, SPOT_HOST};
use crate::types::{Candle, Trade};
use anyhow::{Context, Result};
use reqwest;
use serde_json::Value;
use serde_qs as qs;

pub enum Endpoint {
    Klines,
    AggTrades,
}

impl From<Endpoint> for String {
    fn from(item: Endpoint) -> Self {
        String::from(match item {
            Endpoint::Klines => "/api/v3/klines",
            Endpoint::AggTrades => "/api/v3/aggTrades",
        })
    }
}

pub async fn get_candles(request: CandleRequest) -> Result<Vec<Candle>> {
    let url = format!(
        "{}{}?{}",
        SPOT_HOST,
        String::from(Endpoint::Klines),
        qs::to_string(&request).unwrap()
    );

    println!("url {}", &url);

    let response = reqwest::get(&url)
        .await
        .context("Failed to fetch Binance K-line data")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("Binance API request failed: {} - {}", status, error_message);
    }

    let raw_klines: Vec<Vec<Value>> = response
        .json()
        .await
        .context("Failed to parse JSON response from Binance")?;

    let klines: Vec<Candle> = raw_klines
        .into_iter()
        .map(|k| -> Result<Candle> {
            Ok(Candle {
                open_time: k[0].as_u64().context("Missing open_time")?,
                close_time: k[6].as_u64().context("Missing close_time")?,
                open: k[1].as_str().context("Missing open")?.parse().unwrap(),
                high: k[2].as_str().context("Missing high")?.parse().unwrap(),
                low: k[3].as_str().context("Missing low")?.parse().unwrap(),
                close: k[4].as_str().context("Missing close")?.parse().unwrap(),
                volume: k[5].as_str().context("Missing volume")?.parse().unwrap(),
            })
        })
        .collect::<Result<Vec<Candle>>>()?;

    Ok(klines)
}

pub async fn get_trades(request: TradeRequest) -> Result<Vec<Trade>> {
    let url = format!(
        "{}{}?{}",
        SPOT_HOST,
        String::from(Endpoint::AggTrades),
        qs::to_string(&request).unwrap()
    );

    println!("url {}", &url);

    let response = reqwest::get(&url)
        .await
        .context("Failed to fetch Binance K-line data")?;

    if !response.status().is_success() {
        let status = response.status();
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        anyhow::bail!("Binance API request failed: {} - {}", status, error_message);
    }

    let trades: Vec<Trade> = response
        .json()
        .await
        .context("Failed to parse JSON response from Binance")?;

    Ok(trades)
}
