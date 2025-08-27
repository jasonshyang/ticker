use exstreamer::models::{CoinbaseMessage, CoinbaseTicker};
use tokio_stream::StreamExt as _;

use crate::{
    adapters::ExchangeAdapter,
    error::TickerError,
    types::{Event, EventStream, Exchange, Pair, PairFormat, RawPriceTick},
};

#[derive(Clone)]
pub struct CoinbaseAdapter;

#[async_trait::async_trait]
impl ExchangeAdapter for CoinbaseAdapter {
    fn kind() -> Exchange {
        Exchange::Bybit
    }

    async fn get_event_stream(&self, pair: &Pair) -> Result<EventStream<'_, Event>, TickerError> {
        let (stream, _) = exstreamer::StreamBuilder::coinbase()
            .with_trade(pair.format(PairFormat::UpperWithDash))
            .connect()
            .await?;

        let internal_stream = stream.map(into_event);

        Ok(Box::pin(internal_stream))
    }
}

impl TryFrom<CoinbaseTicker> for RawPriceTick {
    type Error = TickerError;

    fn try_from(tick: CoinbaseTicker) -> Result<Self, Self::Error> {
        let timestamp = chrono::DateTime::parse_from_rfc3339(&tick.time)
            .map_err(|e| TickerError::RawEventParseError(format!("Invalid timestamp: {}", e)))?
            .with_timezone(&chrono::Utc);

        Ok(RawPriceTick {
            price: tick.price.parse()?,
            size: tick.last_size.parse()?,
            timestamp,
        })
    }
}

fn into_event(result: Result<CoinbaseMessage, exstreamer::error::ExStreamError>) -> Event {
    match result {
        Ok(CoinbaseMessage::Ticker(tick)) => match (*tick).try_into() {
            Ok(tick) => Event::PriceTick(tick),
            Err(e) => Event::Error(format!("Failed to parse trade: {}", e)),
        },
        Ok(_) => Event::Unsupported,
        Err(e) => Event::Error(format!("Stream error: {}", e)),
    }
}
