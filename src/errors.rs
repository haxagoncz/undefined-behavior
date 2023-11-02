use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum CustomError {
    #[error("{0}")]
    DbError(#[from] DbErr),
    #[error("Not found")]
    NotFound,
    #[error("User not found")]
    UserNotFound,
    #[error("Unauthorized")]
    Unauthorized,
}

impl IsInternalError for CustomError {
    fn is_internal_error(&self) -> bool {
        match self {
            Self::NotFound => false,
            Self::Unauthorized => false,
            Self::UserNotFound => false,
            _ => true,
        }
    }
}

trait IsInternalError {
    fn is_internal_error(&self) -> bool;
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl IntoResponse for CustomError {
    fn into_response(self) -> Response {
        let message = match self.is_internal_error() {
            true => {
                error!("{}", self);
                "Internal server error".into()
            }
            false => format!("{}", self),
        };
        let status = match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::UserNotFound | Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        };

        let body = serde_json::to_string(&ErrorResponse { error: message })
            .unwrap_or(r#"{ "error": "Internal server error" }"#.into());

        (status, [(header::CONTENT_TYPE, "application/json")], body).into_response()
    }
}
