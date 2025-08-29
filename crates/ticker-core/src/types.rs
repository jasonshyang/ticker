use std::pin::Pin;

use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Serialize;
use tokio_stream::Stream;

use crate::error::TickerError;

pub type EventStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Exchange {
    Binance,
    Bybit,
    Coinbase,
}

#[derive(Debug)]
pub enum Event {
    PriceTick(RawPriceTick),
    Error(String),
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub enum Pair {
    BTCUSDT,
    ETHUSDT,
    SOLUSDT,
}

pub enum PairFormat {
    Upper,
    Lower,
    UpperWithDash,
    LowerWithDash,
}

#[derive(Debug)]
pub struct RawPriceTick {
    pub price: f64,
    pub size: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PriceTick {
    pub exchange: Exchange,
    pub symbol: Pair,
    pub price: f64,
    pub size: f64,
    pub timestamp: DateTime<Utc>,
}

impl PriceTick {
    pub fn try_from_db_record(
        exchange: String,
        symbol: String,
        price: f64,
        size: f64,
        timestamp: NaiveDateTime,
    ) -> Result<Self, TickerError> {
        Ok(Self {
            exchange: exchange.try_into()?,
            symbol: symbol.try_into()?,
            price,
            size,
            timestamp: DateTime::<Utc>::from_naive_utc_and_offset(timestamp, Utc),
        })
    }

    pub fn into_strings(self) -> (String, String, String, String, String) {
        (
            self.exchange.to_string(),
            self.symbol.to_string(),
            self.price.to_string(),
            self.size.to_string(),
            self.timestamp.to_rfc3339(),
        )
    }
}

impl TryFrom<String> for Exchange {
    type Error = TickerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Binance" => Ok(Exchange::Binance),
            "Bybit" => Ok(Exchange::Bybit),
            "Coinbase" => Ok(Exchange::Coinbase),
            _ => Err(TickerError::RawEventParseError(format!(
                "Unknown exchange: {}",
                value
            ))),
        }
    }
}

impl TryFrom<String> for Pair {
    type Error = TickerError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "BTCUSDT" => Ok(Pair::BTCUSDT),
            "ETHUSDT" => Ok(Pair::ETHUSDT),
            "SOLUSDT" => Ok(Pair::SOLUSDT),
            _ => Err(TickerError::RawEventParseError(format!(
                "Unknown pair: {}",
                value
            ))),
        }
    }
}

impl Pair {
    pub fn format(&self, format: PairFormat) -> String {
        match self {
            Pair::BTCUSDT => format.format("BTC", "USDT"),
            Pair::ETHUSDT => format.format("ETH", "USDT"),
            Pair::SOLUSDT => format.format("SOL", "USDT"),
        }
    }
}

impl PairFormat {
    pub fn format(&self, left: &str, right: &str) -> String {
        match self {
            PairFormat::Upper => format!("{}{}", left.to_uppercase(), right.to_uppercase()),
            PairFormat::Lower => format!("{}{}", left.to_lowercase(), right.to_lowercase()),
            PairFormat::UpperWithDash => {
                format!("{}-{}", left.to_uppercase(), right.to_uppercase())
            }
            PairFormat::LowerWithDash => {
                format!("{}-{}", left.to_lowercase(), right.to_lowercase())
            }
        }
    }
}

impl std::fmt::Display for Exchange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Exchange::Binance => "Binance",
            Exchange::Bybit => "Bybit",
            Exchange::Coinbase => "Coinbase",
        };
        write!(f, "{}", s)
    }
}

impl std::fmt::Display for Pair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Pair::BTCUSDT => "BTCUSDT",
            Pair::ETHUSDT => "ETHUSDT",
            Pair::SOLUSDT => "SOLUSDT",
        };
        write!(f, "{}", s)
    }
}
