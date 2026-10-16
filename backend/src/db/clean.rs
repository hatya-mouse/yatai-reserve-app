use chrono::Utc;
use sqlx::SqlitePool;

/// Cleans up the useless entries from the database.
pub(crate) async fn clean_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Remove unused tokens from the database
    let now = Utc::now().timestamp();
    sqlx::query("DELETE FROM auth_tokens WHERE expires_at < ?")
        .bind(now)
        .execute(pool)
        .await?;
    sqlx::query("DELETE FROM status_tokens WHERE expires_at < ?")
        .bind(now)
        .execute(pool)
        .await?;

    Ok(())
}
