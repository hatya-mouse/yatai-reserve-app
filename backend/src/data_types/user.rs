#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct AuthUser {
    pub email: String,
}
