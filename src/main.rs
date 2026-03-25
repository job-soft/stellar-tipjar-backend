use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use std::time::Duration;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use stellar_tipjar_backend::db::connection::AppState;
use stellar_tipjar_backend::services::stellar_service::StellarService;
use stellar_tipjar_backend::{cache, db, email, create_app};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DEBUG: Docker Hot-Reload is working!");
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "stellar_tipjar_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let stellar_rpc_url = std::env::var("STELLAR_RPC_URL")
        .unwrap_or_else(|_| "https://soroban-testnet.stellar.org".to_string());

    let stellar_network = std::env::var("STELLAR_NETWORK")
        .unwrap_or_else(|_| "testnet".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(600))
        .max_lifetime(Duration::from_secs(1800))
        .connect(&database_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    let stellar = StellarService::new(stellar_rpc_url, stellar_network);
    let performance = Arc::new(db::performance::PerformanceMonitor::new());

    // Redis is optional — app starts fine without it, caching is simply skipped.
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let redis = cache::redis_client::connect(&redis_url).await;
    
    // Initialize email system
    let (email_sender, email_rx) = email::sender::EmailSender::new();
    tokio::spawn(email::sender::start_email_worker(email_rx));
    let email_sender = Arc::new(email_sender);

    let state = Arc::new(AppState {
        db: pool,
        stellar,
        performance,
        redis,
        email: email_sender,
    });

    let app = create_app(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Server listening on {}", addr);
    tracing::info!("Swagger UI available at http://{}/swagger-ui", addr);

    axum::serve(listener, app.into_make_service_with_connect_info::<std::net::SocketAddr>()).await?;

    tracing::info!("Server shut down gracefully");
    Ok(())
}
