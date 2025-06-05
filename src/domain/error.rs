use actix_web::{http::StatusCode, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MyError {
    Database(String),
    BadRequest(String),
    NotFound(String),
    Unauthorized(String),
    Validation(String),
    Custom(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Database(e) => write!(f, "Database error: {}", e),
            MyError::BadRequest(msg ) =>  write!(f, "bad reqwest : {}", msg),
            MyError::NotFound(msg) => write!(f, "Not found: {}", msg),
            MyError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            MyError::Validation(msg) => write!(f, "Validation error: {}", msg),
            MyError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl ResponseError for MyError {
    fn status_code(&self) -> StatusCode {
        match self {
            MyError::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::BadRequest(_) => StatusCode::BAD_REQUEST,
            MyError::NotFound(_) => StatusCode::NOT_FOUND,
            MyError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            MyError::Validation(_) => StatusCode::BAD_REQUEST,
            MyError::Custom(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

}
