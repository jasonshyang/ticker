use ticker_core::{error::TickerError, storage::select_price_ticks_after, types::PriceTick};

use crate::config::DURATION_SEC;

pub struct PriceService {
    pub db: sqlx::SqlitePool,
}

impl PriceService {
    pub async fn get_ticks(&self) -> Result<Vec<PriceTick>, TickerError> {
        select_price_ticks_after(&self.db, DURATION_SEC).await
    }
}
