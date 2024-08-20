mod code;

use crate::code::ErrorCode;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0} not found")]
    NotFound(String),
    #[error("{0} not found")]
    ResourceExist(String),
    #[error("hash password failed")]
    PasswordHashFailed(#[from] argon2::password_hash::Error),
    #[error("error occurred while access database")]
    Database(#[from] sqlx::Error),
    #[error("error occurred while build sql statement")]
    SqlBuilder(#[from] sea_query::error::Error),
    #[error("invalid input")]
    InvalidInput(#[from] garde::Report),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    JoinError(#[from] tokio::task::JoinError),
    #[error("build query statement failed:{0}")]
    Query(sea_query::error::Error),
    #[error("you not have permission")]
    PermissionDenied,
    #[error("{0}")]
    Unauthorized(String)
}

impl Error {
    fn response(&self) -> (StatusCode, ResponseError) {
        let message = self.to_string();
        let (status, code, details) = match self {
            Error::NotFound(_) => (StatusCode::NOT_FOUND,ErrorCode::NotFound, None),
            Error::ResourceExist(_) => (StatusCode::CONFLICT,ErrorCode::ResourceAlreadyExists, None),
            Error::PasswordHashFailed(e)=>(StatusCode::INTERNAL_SERVER_ERROR,ErrorCode::PasswordHashFailed,Some(e.to_string())),
            Error::Database(e)=>(StatusCode::INTERNAL_SERVER_ERROR,ErrorCode::DatabaseError,Some(e.to_string())),
            Error::SqlBuilder(e)=>(StatusCode::INTERNAL_SERVER_ERROR,ErrorCode::SqlBuilderError,Some(e.to_string())),
            Error::InvalidInput(e)=>(StatusCode::BAD_REQUEST,ErrorCode::InvalidInput,Some(e.to_string())),
            Error::PermissionDenied=>(StatusCode::FORBIDDEN,ErrorCode::PermissionDenied,None),
            Error::Unauthorized(s)=>(StatusCode::UNAUTHORIZED,ErrorCode::Unauthorized,Some(s.to_string())),
            _=>(StatusCode::INTERNAL_SERVER_ERROR,ErrorCode::InternalServeError,None)
        };
        (status, ResponseError::new(code, &message, details))
    }
}
#[derive(Debug, serde::Serialize)]
struct ResponseError {
    code: ErrorCode,
    message: String,
    details: Option<String>,
}
impl ResponseError {
    pub fn new(code:ErrorCode, message: &str, details: Option<String>) -> Self {
        Self {
            code,
            message: message.to_string(),
            details,
        }
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error) = self.response();
        (status, Json(error)).into_response()
    }
}
pub type Result<T>=std::result::Result<T,Error>;

pub fn invalid_input_error(field: &str, message: &str) -> Error {
    let mut report = garde::Report::new();
    report.append(garde::Path::new(field), garde::Error::new(message));
    Error::InvalidInput(report)
}
