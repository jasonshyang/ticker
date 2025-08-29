use std::{sync::Arc, time::Duration};

use sqlx::SqlitePool;
use ticker_core::{
    adapters::{BinanceAdapter, BybitAdapter, CoinbaseAdapter},
    types::PriceTick,
};
use tokio::{sync::mpsc, task::JoinSet};

pub mod config;
pub mod server;
pub mod services;
pub mod ui;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = SqlitePool::connect("sqlite:./db/prices.db")
        .await
        .expect("Failed to connect to database");

    let (tx, rx) = mpsc::channel::<PriceTick>(config::INTERNAL_CHANNEL_SIZE);
    let tick = Duration::from_millis(config::TICK_INTERVAL_MS);

    let mut set = JoinSet::new();
    set.spawn(ticker_core::storage::run_db_task(db.clone(), rx));
    set.spawn(ticker_core::ingestion::run_ingestion_task(
        tx.clone(),
        BinanceAdapter,
        ticker_core::types::Pair::SOLUSDT,
        config::INGESTION_BUFFER_SIZE,
        tick,
    ));
    set.spawn(ticker_core::ingestion::run_ingestion_task(
        tx.clone(),
        BybitAdapter,
        ticker_core::types::Pair::SOLUSDT,
        config::INGESTION_BUFFER_SIZE,
        tick,
    ));
    set.spawn(ticker_core::ingestion::run_ingestion_task(
        tx.clone(),
        CoinbaseAdapter,
        ticker_core::types::Pair::SOLUSDT,
        config::INGESTION_BUFFER_SIZE,
        tick,
    ));

    let price_service = services::PriceService { db };
    let app_state = server::AppState {
        price: Arc::new(price_service),
    };
    let app = server::create_app(app_state);
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], PORT));
    let listener = tokio::net::TcpListener::bind(addr).await?;

    println!("Server running on http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}
