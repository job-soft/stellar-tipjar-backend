use sqlx::PgPool;

pub async fn check_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query("SELECT 1").fetch_one(pool).await?;
    Ok(())
}
