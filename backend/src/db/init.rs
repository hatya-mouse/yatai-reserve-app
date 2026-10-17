use sqlx::SqlitePool;

pub(crate) async fn init_db(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    // Initialize shops table
    // shops stores the configurations for each shops.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shops (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            notify_previous BOOLEAN NOT NULL DEFAULT 0
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize shop_members table
    // shop_members stores the email addresses and the shop it that the address belongs to.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS shop_members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            shop_id TEXT NOT NULL,
            email TEXT NOT NULL,
            is_admin BOOLEAN NOT NULL DEFAULT 1,
            UNIQUE(shop_id, email)
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize staffs table
    // staffs stores the name and email addresses of each staff.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS staffs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize auth_tokens table
    // auth_tokens stores the currently active tokens for staffs sent over email.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS auth_tokens (
            token TEXT PRIMARY KEY,
            email TEXT NOT NULL,
            expires_at INTEGER NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize status_tokens table
    // status_tokens stores the currently active tokens sent over email for customers to check reservation status.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS status_tokens (
            token TEXT PRIMARY KEY,
            email TEXT NOT NULL,
            expires_at INTEGER NOT NULL
        );
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize sessions table
    // sessions stores the active sessions of staffs.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sessions (
            token TEXT PRIMARY KEY,
            email TEXT NOT NULL,
            expires_at INTEGER NOT NULL
        );

        CREATE INDEX IF NOT EXISTS idx_sessions_expires_at
        ON sessions(expires_at);
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize reservations table
    // reservations stores all information of each reservations.
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reservations (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            shop_id TEXT NOT NULL,
            email TEXT NOT NULL,
            status TEXT NOT NULL,
            created_at INTEGER NOT NULL
        );
        CREATE UNIQUE INDEX IF NOT EXISTS
            idx_reversation_shop_email
        ON reservations (shop_id, email);
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
