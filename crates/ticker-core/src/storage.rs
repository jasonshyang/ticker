use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{error::TickerError, types::PriceTick};

pub async fn run_db_task(
    db: SqlitePool,
    mut rx: mpsc::Receiver<PriceTick>,
) -> Result<(), TickerError> {
    while let Some(tick) = rx.recv().await {
        store_event(&db, tick).await?;
    }
    Ok(())
}

pub async fn create_tables(db: &SqlitePool) -> Result<(), TickerError> {
    sqlx::query(include_str!("../queries/schema.sql"))
        .execute(db)
        .await?;
    Ok(())
}

pub async fn store_event(db: &SqlitePool, tick: PriceTick) -> Result<(), TickerError> {
    let (exchange, symbol, price, size, timestamp) = tick.into_strings();

    sqlx::query_file!(
        "queries/insert_price_tick.sql",
        exchange,
        symbol,
        price,
        size,
        timestamp,
    )
    .execute(db)
    .await?;

    Ok(())
}
