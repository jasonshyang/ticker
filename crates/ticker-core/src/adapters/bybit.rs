use exstreamer::models::{BybitMessage, BybitTradeData};
use futures::{StreamExt, stream};

use crate::{
    adapters::ExchangeAdapter,
    error::TickerError,
    types::{Event, EventStream, Exchange, Pair, PairFormat, PriceTick},
};

#[derive(Clone)]
pub struct BybitAdapter;

#[async_trait::async_trait]
impl ExchangeAdapter for BybitAdapter {
    async fn get_event_stream(&self, pair: &Pair) -> Result<EventStream<'_, Event>, TickerError> {
        let (stream, _) = exstreamer::StreamBuilder::bybit()
            .with_trade(pair.to_string(PairFormat::Lower))
            .connect()
            .await?;

        let internal_stream = stream.flat_map(into_event_flatten);

        Ok(Box::pin(internal_stream))
    }
}

impl TryFrom<BybitTradeData> for PriceTick {
    type Error = TickerError;

    fn try_from(trade: BybitTradeData) -> Result<Self, Self::Error> {
        Ok(PriceTick {
            exchange: Exchange::Bybit,
            symbol: trade.symbol,
            price: trade.price.parse()?,
            size: trade.size.parse()?,
            timestamp: chrono::DateTime::from_timestamp_millis(trade.timestamp as i64)
                .ok_or_else(|| TickerError::RawEventParseError("Invalid timestamp".to_string()))?,
        })
    }
}

fn into_event_flatten(
    result: Result<BybitMessage, exstreamer::error::ExStreamError>,
) -> EventStream<'static, Event> {
    match result {
        Ok(BybitMessage::Trade(trades)) => Box::pin(stream::iter(trades.data.into_iter().map(
            |trade| match trade.try_into() {
                Ok(tick) => Event::PriceTick(tick),
                Err(e) => Event::Error(e.to_string()),
            },
        ))),
        Err(e) => Box::pin(stream::once(async move { Event::Error(e.to_string()) })),
        _ => Box::pin(stream::once(async { Event::Unsupported })),
    }
}
