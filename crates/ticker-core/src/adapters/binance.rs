use exstreamer::models::{BinanceMessage, BinanceTrade};
use tokio_stream::StreamExt as _;

use crate::{
    adapters::ExchangeAdapter,
    error::TickerError,
    types::{Event, EventStream, Exchange, Pair, PairFormat, RawPriceTick},
};

#[derive(Clone)]
pub struct BinanceAdapter;

#[async_trait::async_trait]
impl ExchangeAdapter for BinanceAdapter {
    fn kind() -> Exchange {
        Exchange::Binance
    }

    async fn get_event_stream(&self, pair: &Pair) -> Result<EventStream<'_, Event>, TickerError> {
        let (stream, _) = exstreamer::StreamBuilder::binance()
            .with_trade(pair.format(PairFormat::Lower))
            .connect()
            .await?;

        let internal_stream = stream.map(into_event);

        Ok(Box::pin(internal_stream))
    }
}

impl TryFrom<BinanceTrade> for RawPriceTick {
    type Error = TickerError;

    fn try_from(trade: BinanceTrade) -> Result<Self, Self::Error> {
        Ok(RawPriceTick {
            price: trade.price.parse()?,
            size: trade.quantity.parse()?,
            timestamp: chrono::DateTime::from_timestamp_millis(trade.trade_time as i64)
                .ok_or_else(|| TickerError::RawEventParseError("Invalid timestamp".to_string()))?,
        })
    }
}

fn into_event(result: Result<BinanceMessage, exstreamer::error::ExStreamError>) -> Event {
    match result {
        Ok(BinanceMessage::Trade(trade)) => match trade.try_into() {
            Ok(price_tick) => Event::PriceTick(price_tick),
            Err(e) => Event::Error(format!("Failed to parse trade: {}", e)),
        },
        Ok(_) => Event::Unsupported,
        Err(e) => Event::Error(format!("Stream error: {}", e)),
    }
}
