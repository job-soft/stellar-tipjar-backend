use redis::aio::ConnectionManager;
use sqlx::PgPool;
use std::sync::Arc;

use crate::services::stellar_service::StellarService;
use super::performance::PerformanceMonitor;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub stellar: StellarService,
    pub performance: Arc<PerformanceMonitor>,
    pub redis: Option<ConnectionManager>,
    pub email: Arc<crate::email::EmailSender>,
    pub tip_service: Arc<crate::services::tip_service::TipService>,
    pub creator_service: Arc<crate::services::creator_service::CreatorService>,
}
