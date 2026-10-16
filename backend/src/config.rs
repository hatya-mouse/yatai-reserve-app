pub(super) struct Config {
    /// The host of the server to listen on.
    pub(super) host: String,
    /// The port for the server to listen on.
    pub(super) port: u16,
    /// An URL to the SQLite database.
    pub(super) database_url: String,
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
        }
    }
}
