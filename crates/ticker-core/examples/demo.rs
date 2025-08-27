use std::time::Duration;

use sqlx::SqlitePool;
use ticker_core::{adapters::BinanceAdapter, types::PriceTick};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let db = SqlitePool::connect("sqlite:./db/prices.db")
        .await
        .expect("Failed to connect to database");

    let (tx, rx) = mpsc::channel::<PriceTick>(100);
    let db_fut = ticker_core::storage::run_db_task(db, rx);
    let ingestion_fut = ticker_core::ingestion::run_ingestion_task(
        tx,
        BinanceAdapter,
        ticker_core::types::Pair::SOLUSDT,
        100_000,
        Duration::from_millis(100),
    );

    tokio::select! {
        res = db_fut => {
            if let Err(e) = res {
                eprintln!("Database task error: {}", e);
            }
        },
        res = ingestion_fut => {
            if let Err(e) = res {
                eprintln!("Ingestion task error: {}", e);
            }
        },
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        },
    }

    println!("Demo finished.");
}
