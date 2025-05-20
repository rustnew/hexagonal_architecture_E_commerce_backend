use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    Database(sqlx::Error),
    Validation(String),
    Unauthorized(String),
    NotFound(String),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::Database(e) => write!(f, "Database error: {}", e),
            MyError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MyError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            MyError::NotFound(msg) => write!(f, "Not found: {}", msg),
            MyError::Custom(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            MyError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            MyError::Validation(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            MyError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg.clone()),
            MyError::NotFound(msg) => (StatusCode::NOT_FOUND, msg.clone()),
            MyError::Custom(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg.clone()),
        };

        let body = serde_json::json!({ "error": message });
        (status, axum::Json(body)).into_response()
    }
}

pub fn validation_error(msg: &str) -> MyError {
    MyError::Validation(msg.to_string())
}

pub fn not_found_error(msg: &str) -> MyError {
    MyError::NotFound(msg.to_string())
}