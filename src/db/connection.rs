use redis::aio::ConnectionManager;
use sqlx::PgPool;

use crate::services::stellar_service::StellarService;

pub struct AppState {
    pub db: PgPool,
    pub stellar: StellarService,
    /// None when Redis is unavailable — controllers fall back to DB directly.
    pub redis: Option<ConnectionManager>,
}
