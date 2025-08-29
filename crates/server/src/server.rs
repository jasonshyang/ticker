use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, routing::get};
use ticker_core::types::PriceTick;

use crate::{services::PriceService, ui::index_page};

#[derive(Clone)]
pub struct AppState {
    pub price: Arc<PriceService>,
}

pub fn create_app(state: AppState) -> axum::Router {
    axum::Router::new()
        .route("/ticks", get(get_ticks))
        .route("/ui", get(index_page))
        .with_state(state)
}

pub async fn get_ticks(State(state): State<AppState>) -> Result<Json<Vec<PriceTick>>, StatusCode> {
    println!("Received request for price ticks");
    let res = state.price.get_ticks().await.map(Json);

    match res {
        Ok(json) => Ok(json),
        Err(e) => {
            eprintln!("Error fetching price ticks: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
