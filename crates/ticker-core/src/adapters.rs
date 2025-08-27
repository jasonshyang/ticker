mod binance;
mod bybit;
mod coinbase;

pub use binance::*;
pub use bybit::*;
pub use coinbase::*;

use crate::{
    error::TickerError,
    types::{Event, EventStream, Pair},
};

#[async_trait::async_trait]
pub trait ExchangeAdapter {
    async fn get_event_stream(&self, pair: &Pair) -> Result<EventStream<'_, Event>, TickerError>;
}
