use anyhow::Result;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use uuid::Uuid;

use crate::cache::{keys, redis_client};
use crate::models::tip::{RecordTipRequest, Tip};

pub async fn record_tip(
    pool: &PgPool,
    redis: &Option<ConnectionManager>,
    req: RecordTipRequest,
) -> Result<Tip> {
    let tip = sqlx::query_as::<_, Tip>(
        r#"
        INSERT INTO tips (id, creator_username, amount, transaction_hash, created_at)
        VALUES ($1, $2, $3, $4, NOW())
        RETURNING id, creator_username, amount, transaction_hash, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&req.username)
    .bind(&req.amount)
    .bind(&req.transaction_hash)
    .fetch_one(pool)
    .await?;

    // Invalidate the tip list cache for this creator since it's now stale.
    if let Some(conn) = redis.as_ref() {
        let mut conn = conn.clone();
        let tips_key = keys::creator_tips(&tip.creator_username);
        redis_client::del(&mut conn, &[tips_key.as_str()]).await;
    }

    Ok(tip)
}

pub async fn get_tips_for_creator(
    pool: &PgPool,
    redis: &Option<ConnectionManager>,
    username: &str,
) -> Result<Vec<Tip>> {
    let cache_key = keys::creator_tips(username);

    // Try cache first.
    if let Some(conn) = redis.as_ref() {
        let mut conn = conn.clone();
        if let Some(cached) = redis_client::get::<Vec<Tip>>(&mut conn, &cache_key).await {
            tracing::debug!("Cache hit: {}", cache_key);
            return Ok(cached);
        }
    }

    // Cache miss — hit the DB.
    let tips = sqlx::query_as::<_, Tip>(
        r#"
        SELECT id, creator_username, amount, transaction_hash, created_at
        FROM tips
        WHERE creator_username = $1
        ORDER BY created_at DESC
        "#,
    )
    .bind(username)
    .fetch_all(pool)
    .await?;

    // Populate cache.
    if let Some(conn) = redis.as_ref() {
        let mut conn = conn.clone();
        redis_client::set(&mut conn, &cache_key, &tips, redis_client::TTL_TIPS).await;
    }

    Ok(tips)
}
