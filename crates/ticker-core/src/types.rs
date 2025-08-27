use std::pin::Pin;

use chrono::{DateTime, Utc};
use dec::Decimal64;
use tokio_stream::Stream;

pub type EventStream<'a, E> = Pin<Box<dyn Stream<Item = E> + Send + 'a>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Exchange {
    Binance,
    Bybit,
    Coinbase,
}

#[derive(Debug)]
pub enum Event {
    PriceTick(PriceTick),
    Error(String),
    Unsupported,
}

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
pub struct PriceTick {
    pub exchange: Exchange,
    pub symbol: String,
    pub price: Decimal64,
    pub size: Decimal64,
    pub timestamp: DateTime<Utc>,
}

impl Pair {
    pub fn to_string(&self, format: PairFormat) -> String {
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
