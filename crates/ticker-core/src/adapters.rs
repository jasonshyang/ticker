mod binance;
mod bybit;
mod coinbase;

pub use binance::*;
pub use bybit::*;
pub use coinbase::*;

use crate::{
    error::TickerError,
    types::{Event, EventStream, Exchange, Pair},
};

#[async_trait::async_trait]
pub trait ExchangeAdapter {
    fn kind() -> Exchange;
    async fn get_event_stream(&self, pair: &Pair) -> Result<EventStream<'_, Event>, TickerError>;
}
