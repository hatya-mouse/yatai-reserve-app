use crate::tui::println_error;
use axum::{
    http::{StatusCode, header::InvalidHeaderValue},
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub(crate) enum AppError {
    Unauthorized,
    BadRequest,
    NotFound,
    Conflict,
    Forbidden,
    SqlxError(sqlx::Error),
    Internal,
}

impl From<sqlx::Error> for AppError {
    fn from(error: sqlx::Error) -> Self {
        Self::SqlxError(error)
    }
}

impl From<InvalidHeaderValue> for AppError {
    fn from(_: InvalidHeaderValue) -> Self {
        Self::Internal
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match &self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::BadRequest => StatusCode::BAD_REQUEST,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::Conflict => StatusCode::CONFLICT,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::SqlxError(error) => {
                println_error(format!("Internal error: {error:#?}"));
                StatusCode::INTERNAL_SERVER_ERROR
            }
            Self::Internal => StatusCode::INTERNAL_SERVER_ERROR,
        };

        status.into_response()
    }
}
