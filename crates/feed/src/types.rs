use anyhow::{bail, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct Candle {
    pub open_time: u64,
    pub close_time: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Trade {
    #[serde(rename = "T")]
    pub timestamp: u64,

    #[serde(rename = "p", deserialize_with = "string_to_f64")]
    pub price: f64,

    #[serde(rename = "q", deserialize_with = "string_to_f64")]
    pub qty: f64,

    #[serde(rename = "m")]
    pub is_buyer_maker: bool,
}

fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Interval {
    OneSecond,
    OneMinute,
    ThreeMinutes,
    FiveMinutes,
    FifteenMinutes,
    ThirtyMinutes,
    OneHour,
    TwoHours,
    FourHours,
    SixHours,
    EightHours,
    TwelveHours,
    OneDay,
    ThreeDays,
    OneWeek,
    OneMonth,
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "1s" => Ok(Interval::OneSecond),
            "1m" => Ok(Interval::OneMinute),
            "3m" => Ok(Interval::ThreeMinutes),
            "5m" => Ok(Interval::FiveMinutes),
            "15m" => Ok(Interval::FifteenMinutes),
            "30m" => Ok(Interval::ThirtyMinutes),
            "1h" => Ok(Interval::OneHour),
            "2h" => Ok(Interval::TwoHours),
            "4h" => Ok(Interval::FourHours),
            "6h" => Ok(Interval::SixHours),
            "8h" => Ok(Interval::EightHours),
            "12h" => Ok(Interval::TwelveHours),
            "1d" => Ok(Interval::OneDay),
            "3d" => Ok(Interval::ThreeDays),
            "1w" => Ok(Interval::OneWeek),
            "1M" => Ok(Interval::OneMonth),
            _ => bail!("Unknown Interval Type"),
        }
    }
}

impl From<Interval> for String {
    fn from(item: Interval) -> Self {
        String::from(match item {
            Interval::OneSecond => "1s",
            Interval::OneMinute => "1m",
            Interval::ThreeMinutes => "3m",
            Interval::FiveMinutes => "5m",
            Interval::FifteenMinutes => "15m",
            Interval::ThirtyMinutes => "30m",
            Interval::OneHour => "1h",
            Interval::TwoHours => "2h",
            Interval::FourHours => "4h",
            Interval::SixHours => "6h",
            Interval::EightHours => "8h",
            Interval::TwelveHours => "12h",
            Interval::OneDay => "1d",
            Interval::ThreeDays => "3d",
            Interval::OneWeek => "1w",
            Interval::OneMonth => "1M",
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Exchange {
    Binance,
    Bybit,
    Kucoin,
    GateIO,
    Kraken,
    MEXC,
    OKX,
    Indodax,
}

impl FromStr for Exchange {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "binance" => Ok(Exchange::Binance),
            "bybit" => Ok(Exchange::Bybit),
            "kucoin" => Ok(Exchange::Kucoin),
            "gateio" => Ok(Exchange::GateIO),
            "kraken" => Ok(Exchange::Kraken),
            "mexc" => Ok(Exchange::MEXC),
            "okx" => Ok(Exchange::OKX),
            "indodax" => Ok(Exchange::Indodax),
            _ => bail!("Unknown Exchange Type"),
        }
    }
}

impl From<Exchange> for String {
    fn from(item: Exchange) -> Self {
        String::from(match item {
            Exchange::Binance => "binance",
            Exchange::Bybit => "bybit",
            Exchange::Kucoin => "kucoin",
            Exchange::GateIO => "gateio",
            Exchange::Kraken => "kraken",
            Exchange::MEXC => "mexc",
            Exchange::OKX => "okx",
            Exchange::Indodax => "indodax",
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Exchange, Interval};

    #[test]
    fn interval() {
        let curr_interval = Interval::OneDay;
        let my_interval: Interval = "1w".parse().unwrap();

        assert_eq!(String::from(curr_interval), "1d");
        assert_eq!(my_interval, Interval::OneWeek);
    }

    #[test]
    fn exchange() {
        let curr_exchange = Exchange::Indodax;
        let my_exchange: Exchange = "indodax".parse().unwrap();

        assert_eq!(String::from(curr_exchange), "indodax");
        assert_eq!(my_exchange, Exchange::Indodax);
    }
}
