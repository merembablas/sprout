use serde::{Deserialize, Serialize};

pub const SPOT_HOST: &str = "https://api.binance.com";

#[derive(Serialize, Deserialize)]
pub struct CandleRequest {
    pub symbol: String,

    pub interval: String,

    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,

    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,

    #[serde(rename = "timeZone")]
    pub timezone: Option<String>,

    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct TradeRequest {
    pub symbol: String,

    #[serde(rename = "startTime")]
    pub start_time: Option<i64>,

    #[serde(rename = "endTime")]
    pub end_time: Option<i64>,

    pub limit: Option<u64>,
}
