use std::time::Duration;

use chrono::{DateTime, Utc};
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;

use crate::{
    adapters::ExchangeAdapter,
    error::TickerError,
    types::{Event, Exchange, Pair, PriceTick},
};

pub async fn run_ingestion_task<E>(
    tx: mpsc::Sender<PriceTick>,
    exchange: E,
    pair: Pair,
    buffer_size: usize,
    tick: Duration,
) -> Result<(), TickerError>
where
    E: ExchangeAdapter + Send + Sync + 'static,
{
    let mut stream = match exchange.get_event_stream(&pair).await {
        Ok(s) => s,
        Err(e) => {
            eprintln!(
                "Error getting event stream for {:?} on {:?}: {}",
                pair,
                E::kind(),
                e
            );
            return Err(e);
        }
    };
    let mut buffer = Vec::with_capacity(buffer_size);
    let mut ticker = tokio::time::interval(tick);

    loop {
        tokio::select! {
            Some(event) = stream.next() => {
                buffer.push(event);
                if buffer.len() > buffer_size {
                    eprintln!("Warning: buffer size exceeded")
                }
            }
            _ = ticker.tick() => {
                let events = std::mem::take(&mut buffer);
                if let Some(price_tick) = par_aggregate(E::kind(), pair, Utc::now(), events).await {
                    if tx.send(price_tick).await.is_err() {
                        eprintln!("Receiver dropped, stopping ingestion task for {:?} on {:?}", pair, E::kind());
                        break;
                    }
                }
            }
        }
    }

    Ok(())
}

async fn par_aggregate(
    exchange: Exchange,
    pair: Pair,
    ts: DateTime<Utc>,
    events: Vec<Event>,
) -> Option<PriceTick> {
    let (weighted_sum_price, total_size) = events
        .into_par_iter()
        .filter_map(|event| match event {
            Event::PriceTick(tick) if tick.price > 0.0 && tick.size > 0.0 => {
                Some((tick.price * tick.size, tick.size))
            }
            Event::Error(err) => {
                eprintln!("Error event from {:?} on {:?}: {}", exchange, pair, err);
                None
            }
            _ => None,
        })
        .reduce(
            || (0.0, 0.0),
            |(sum_vw, sum_w), (vw, w)| (sum_vw + vw, sum_w + w),
        );

    if total_size > 0.0 {
        Some(PriceTick {
            exchange,
            symbol: pair,
            price: weighted_sum_price / total_size,
            size: total_size,
            timestamp: ts,
        })
    } else {
        None
    }
}
