use anyhow::Result;
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::creator::{CreateCreatorRequest, Creator};
use crate::search::SearchQuery;

pub async fn create_creator(pool: &PgPool, req: CreateCreatorRequest) -> Result<Creator> {
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

    Ok(creator)
}

pub async fn get_creator_by_username(pool: &PgPool, username: &str) -> Result<Option<Creator>> {
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

    Ok(creator)
}

/// Search creators by username using PostgreSQL full-text search with trigram
/// fuzzy fallback. Results are ranked by ts_rank descending.
pub async fn search_creators(pool: &PgPool, query: &SearchQuery) -> Result<Vec<Creator>> {
    let term = query.q.trim().to_string();
    let limit = query.clamped_limit();

    let creators = sqlx::query_as::<_, Creator>(
        r#"
        SELECT id, username, wallet_address, created_at
        FROM creators
        WHERE
            search_vector @@ plainto_tsquery('english', $1)
            OR username ILIKE '%' || $1 || '%'
        ORDER BY
            ts_rank(search_vector, plainto_tsquery('english', $1)) DESC,
            created_at DESC
        LIMIT $2
        "#,
    )
    .bind(&term)
    .bind(limit)
    .fetch_all(pool)
    .await?;

    Ok(creators)
}
