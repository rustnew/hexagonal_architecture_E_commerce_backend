use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use sqlx::Error as SqlxError;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Database(SqlxError),
    NotFound(String),
    Unauthorized(String),
    Validation(String),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Database(e) => write!(f, "Database error: {}", e),
            MyError::NotFound(msg) => write!(f, "Not found: {}", msg),
            MyError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            MyError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MyError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl std::error::Error for MyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            MyError::Database(e) => Some(e),
            _ => None,
        }
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            MyError::Validation(_) => StatusCode::BAD_REQUEST,
            MyError::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(serde_json::json!({
            "error": self.to_string()
        }))
    }
}

pub fn not_found_error(msg: &str) -> MyError {
    MyError::NotFound(msg.to_string())
}

pub fn validation_error(msg: &str) -> MyError {
    MyError::Validation(msg.to_string())
}