use anyhow::Result;
use redis::aio::ConnectionManager;
use sqlx::PgPool;
use uuid::Uuid;

use crate::cache::{keys, redis_client};
use crate::models::creator::{CreateCreatorRequest, Creator};

pub async fn create_creator(
    pool: &PgPool,
    redis: &Option<ConnectionManager>,
    req: CreateCreatorRequest,
) -> Result<Creator> {
    let creator = sqlx::query_as::<_, Creator>(
        r#"
        INSERT INTO creators (id, username, wallet_address, created_at)
        VALUES ($1, $2, $3, NOW())
        RETURNING id, username, wallet_address, created_at
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&req.username)
    .bind(&req.wallet_address)
    .fetch_one(pool)
    .await?;

    // Warm the cache immediately after creation.
    if let Some(conn) = redis.as_ref() {
        let mut conn = conn.clone();
        redis_client::set(&mut conn, &keys::creator(&creator.username), &creator, redis_client::TTL_CREATOR).await;
    }

    Ok(creator)
}

pub async fn get_creator_by_username(
    pool: &PgPool,
    redis: &Option<ConnectionManager>,
    username: &str,
) -> Result<Option<Creator>> {
    // Try cache first.
    if let Some(conn) = redis.as_ref() {
        let mut conn = conn.clone();
        if let Some(cached) = redis_client::get::<Creator>(&mut conn, &keys::creator(username)).await {
            tracing::debug!("Cache hit: creator:{}", username);
            return Ok(Some(cached));
        }
    }

    // Cache miss — hit the DB.
    let creator = sqlx::query_as::<_, Creator>(
        r#"
        SELECT id, username, wallet_address, created_at
        FROM creators
        WHERE username = $1
        "#,
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;

    // Populate cache if found.
    if let (Some(ref c), Some(conn)) = (&creator, redis.as_ref()) {
        let mut conn = conn.clone();
        redis_client::set(&mut conn, &keys::creator(username), c, redis_client::TTL_CREATOR).await;
    }

    Ok(creator)
}
