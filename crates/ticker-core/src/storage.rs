use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{error::TickerError, types::Event};

pub async fn run_db_task(db: SqlitePool, mut rx: mpsc::Receiver<Event>) -> Result<(), TickerError> {
    while let Some(event) = rx.recv().await {
        store_event(event, &db).await?;
    }
    Ok(())
}

async fn store_event(event: Event, db: &SqlitePool) -> Result<(), TickerError> {
    if let Some(query) = insert_query(&event) {
        sqlx::query(&query).execute(db).await?;
    }
    Ok(())
}

fn insert_query(event: &Event) -> Option<String> {
    match event {
        Event::PriceTick(tick) => Some(format!(
            "INSERT INTO price_ticks (exchange, symbol, price, sz, ts) VALUES ('{}', '{}', {}, {}, '{}');",
            tick.exchange, tick.symbol, tick.price, tick.size, tick.timestamp
        )),
        Event::Error(err) => {
            eprintln!("Error event received: {}", err);
            None
        }
        Event::Unsupported => {
            eprintln!("Unsupported event received");
            None
        }
    }
}
