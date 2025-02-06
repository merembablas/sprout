use anyhow::Result;
use chrono::{NaiveDate, Utc};
use clap::{Parser, Subcommand};
use csv::Writer;
use feed::binance::config::{CandleRequest, TradeRequest};
use feed::binance::spot::{get_candles, get_trades};
use feed::types::Exchange;

#[derive(Parser)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    ExportCandle {
        #[clap(short, long)]
        symbol: String,

        #[clap(short, long)]
        exchange: String,

        #[clap(short, long)]
        market: String,

        #[clap(short, long)]
        interval: String,

        #[clap(long)]
        start_date: Option<String>,

        #[clap(long)]
        end_date: Option<String>,

        #[clap(short, long)]
        output: Option<String>,
    },

    ExportTrade {
        #[clap(short, long)]
        symbol: String,

        #[clap(short, long)]
        exchange: String,

        #[clap(short, long)]
        market: String,

        #[clap(long)]
        start_date: Option<String>,

        #[clap(long)]
        end_date: Option<String>,

        #[clap(short, long)]
        output: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match &args.command {
        Some(Commands::ExportCandle {
            symbol,
            exchange,
            market,
            interval,
            start_date,
            end_date,
            output,
        }) => {
            let start = if let Some(start_date) = start_date {
                let ns = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
                    .expect("Invalid start date format");

                let s = chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                    ns.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                );
                Some(s.timestamp_millis())
            } else {
                None
            };

            let end = if let Some(end_date) = end_date {
                let ne = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
                    .expect("Invalid end date format");

                let e = chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                    ne.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                );

                Some(e.timestamp_millis())
            } else {
                None
            };

            let file = if let Some(output) = output {
                output.clone()
            } else {
                format!("Candles_{}_{}_{}.csv", exchange, symbol, market)
            };

            let request = CandleRequest {
                symbol: symbol.clone(),
                interval: interval.clone(),
                start_time: start,
                end_time: end,
                timezone: None,
                limit: None,
            };

            let result = match exchange.parse().unwrap() {
                Exchange::Binance => get_candles(request).await,
                Exchange::Bybit => get_candles(request).await,
                Exchange::Kucoin => get_candles(request).await,
                Exchange::GateIO => get_candles(request).await,
                Exchange::Kraken => get_candles(request).await,
                Exchange::MEXC => get_candles(request).await,
                Exchange::OKX => get_candles(request).await,
                Exchange::Indodax => get_candles(request).await,
            };

            if let Ok(result) = result {
                let mut csv_file = Writer::from_path(file)?;

                for i in result {
                    let _ = csv_file.serialize(i);
                }
            }
        }

        Some(Commands::ExportTrade {
            symbol,
            exchange,
            market,
            start_date,
            end_date,
            output,
        }) => {
            let start = if let Some(start_date) = start_date {
                let ns = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
                    .expect("Invalid start date format");

                let s = chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                    ns.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                );

                Some(s.timestamp_millis())
            } else {
                None
            };

            let end = if let Some(end_date) = end_date {
                let ne = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
                    .expect("Invalid end date format");

                let e = chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                    ne.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                );

                Some(e.timestamp_millis())
            } else {
                None
            };

            let file = if let Some(output) = output {
                output.clone()
            } else {
                format!("Trades_{}_{}_{}.csv", exchange, symbol, market)
            };

            let request = TradeRequest {
                symbol: symbol.clone(),
                start_time: start,
                end_time: end,
                limit: None,
            };

            let result = match exchange.parse().unwrap() {
                Exchange::Binance => get_trades(request).await,
                Exchange::Bybit => get_trades(request).await,
                Exchange::Kucoin => get_trades(request).await,
                Exchange::GateIO => get_trades(request).await,
                Exchange::Kraken => get_trades(request).await,
                Exchange::MEXC => get_trades(request).await,
                Exchange::OKX => get_trades(request).await,
                Exchange::Indodax => get_trades(request).await,
            };

            if let Ok(result) = result {
                let mut csv_file = Writer::from_path(file)?;

                for i in result {
                    let _ = csv_file.serialize(i);
                }
            } else {
                println!("error {:?}", result);
            }
        }

        None => {}
    }

    Ok(())
}
