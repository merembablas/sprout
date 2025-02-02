use crate::Candle;
use anyhow::{Context, Result};
use chrono::{DateTime, TimeZone, Utc};
use reqwest;

pub async fn get_historical_candles() -> Result<()> {
    let symbol = "BTCUSDT"; // Trading pair
    let interval = "1h"; // K-line interval
    let limit = 5; // Number of candles

    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&limit={}",
        symbol, interval, limit
    );

    let response: Vec<Vec<serde_json::Value>> = reqwest::get(&url)
        .await
        .context("Failed to fetch Binance K-line data")?
        .json()
        .await
        .context("Failed to parse JSON response")?;
    /*
    let klines: Vec<Candle> = response
        .into_iter()
        .map(|k| -> Result<Candle> {
            Ok(Candle {
                open_time: k[0].as_u64().context("Missing open_time")?,
                close_time: k[1].as_u64().context("Missing close_time")?,
                open: k[2].as_f64().context("Missing open")?.to_string(),
                high: k[3].as_f64().context("Missing high")?.to_string(),
                low: k[4].as_f64().context("Missing low")?.to_string(),
                close: k[5].as_f64().context("Missing close")?.to_string(),
                volume: k[6].as_f64().context("Missing volume")?.to_string(),
                interval: k[7].as_str().context("Missing close")?.to_string(),
                symbol: k[8].as_str().context("Missing volume")?.to_string(),
            })
        })
        .collect::<Result<Vec<Candle>>>()?; // Collect into Result<Vec<Kline>>

    for kline in &klines {
        let dt = TimeZone::<Utc>::from_utc(
            DateTime::from_timestamp_millis(kline.open_time).context("Invalid timestamp")?,
            Utc,
        );
        println!("{} | Open: {}, Close: {}", dt, kline.open, kline.close);
    }
    */
    Ok(())
}
