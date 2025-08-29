use sqlx::SqlitePool;
use tokio::sync::mpsc;

use crate::{error::TickerError, types::PriceTick};

pub async fn run_db_task(
    db: SqlitePool,
    mut rx: mpsc::Receiver<PriceTick>,
) -> Result<(), TickerError> {
    create_tables(&db).await?;

    while let Some(tick) = rx.recv().await {
        match store_event(&db, tick).await {
            Ok(_) => (),
            Err(e) => eprintln!("Error storing price tick: {}", e),
        }
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

pub async fn select_price_ticks(
    db: &SqlitePool,
    exchange: &str,
    symbol: &str,
    limit: i64,
) -> Result<Vec<PriceTick>, TickerError> {
    let rows = sqlx::query_file!("queries/select_price_ticks.sql", exchange, symbol, limit)
        .fetch_all(db)
        .await?;

    let ticks = rows
        .into_iter()
        .map(|row| {
            PriceTick::try_from_db_record(row.exchange, row.symbol, row.price, row.sz, row.ts)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ticks)
}

pub async fn select_all_price_ticks(
    db: &SqlitePool,
    limit: i64,
) -> Result<Vec<PriceTick>, TickerError> {
    let rows = sqlx::query_file!("queries/select_all_price_ticks.sql", limit)
        .fetch_all(db)
        .await?;

    let ticks = rows
        .into_iter()
        .map(|row| {
            PriceTick::try_from_db_record(row.exchange, row.symbol, row.price, row.sz, row.ts)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ticks)
}

pub async fn select_price_ticks_after(
    db: &SqlitePool,
    secs: i64,
) -> Result<Vec<PriceTick>, TickerError> {
    let ts = chrono::Utc::now().naive_utc() - chrono::Duration::seconds(secs);
    let rows = sqlx::query_file!("queries/select_price_ticks_after.sql", ts)
        .fetch_all(db)
        .await?;

    let ticks = rows
        .into_iter()
        .map(|row| {
            PriceTick::try_from_db_record(row.exchange, row.symbol, row.price, row.sz, row.ts)
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ticks)
}
