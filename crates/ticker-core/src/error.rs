use std::num::ParseFloatError;

#[derive(Debug, thiserror::Error)]
pub enum TickerError {
    #[error("Exchange stream error: {0}")]
    StreamError(#[from] exstreamer::error::ExStreamError),
    #[error("Failed to parse raw event: {0}")]
    RawEventParseError(String),
    #[error("Float parse error: {0}")]
    ParseDecimalError(#[from] ParseFloatError),
    #[error("Channel closed")]
    ChannelClosed,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}
