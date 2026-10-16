use sqlx::SqlitePool;

/// A struct to store the application state that is used across handlers.
#[derive(Debug, Clone)]
pub(super) struct AppState {
    pub(super) pool: SqlitePool,
    pub(super) config: Config,
}

/// The configuration of the backend.
#[derive(Debug, Clone)]
pub(super) struct Config {
    /// The host of the server to listen on.
    pub(super) host: String,
    /// The port for the server to listen on.
    pub(super) port: u16,
    /// An URL to the SQLite database.
    pub(super) database_url: String,
    /// The URL of the verification page of the frontend.
    pub(super) verification_url: String,
    /// The URL that the staff is redirected to after verification.
    pub(super) home_url: String,
    /// Resend API key to send emails.
    pub(super) resend_key: String,
    /// An email address to send emails from.
    pub(super) email: String,
}

impl Config {
    pub(super) fn load() -> Self {
        Self {
            host: std::env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
            port: std::env::var("PORT")
                .unwrap_or_else(|_| String::from("3000"))
                .parse()
                .expect("Port number must be an unsigned 16 bit integer"),
            database_url: std::env::var("DATABASE_URL").unwrap_or_default(),
            verification_url: std::env::var("VERIFICATION_URL").unwrap_or_default(),
            home_url: std::env::var("HOME_URL").unwrap_or_default(),
            resend_key: std::env::var("RESEND_KEY").unwrap_or_default(),
            email: std::env::var("EMAIL").unwrap_or_default(),
        }
    }
}
