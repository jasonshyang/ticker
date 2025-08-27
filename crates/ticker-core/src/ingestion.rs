use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    adapters::ExchangeAdapter,
    error::TickerError,
    types::{Event, Pair},
};

pub async fn run_ingestion_task<Ex>(
    tx: mpsc::Sender<Event>,
    exchange: Ex,
    pair: Pair,
) -> Result<(), TickerError>
where
    Ex: ExchangeAdapter + Send + Sync + 'static,
{
    let mut stream = exchange.get_event_stream(&pair).await?;
    while let Some(event) = stream.next().await {
        tx.send(event)
            .await
            .map_err(|_| TickerError::ChannelClosed)?;
    }

    Ok(())
}
