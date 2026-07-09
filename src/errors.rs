use std::str::Utf8Error;

use actix_multipart::MultipartError;
// src/errors.rs
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Internal server error: {0}")]
    InternalError(String),

    #[error("Invalid id")]
    InvalidId(String),

    #[error("Cache error")]
    RedisError(String),

    #[error("AI service error")]
    AIServiceError(String),
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::AuthenticationError(_) => {
                HttpResponse::Unauthorized().json(json_error_response(&self.to_string()))
            }
            AppError::ValidationError(_) => {
                HttpResponse::BadRequest().json(json_error_response(&self.to_string()))
            }
            AppError::ConfigError(_) => {
                log::error!("Config error: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("Server config error"))
            }
            AppError::DatabaseError(_) | AppError::InternalError(_) => {
                log::error!("Internal error: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("Internal server error"))
            }
            AppError::InvalidId(_) => {
                log::error!("Client error: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("Invalid id"))
            }
            AppError::RedisError(_) => {
                log::error!("Cache error: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("Cache service error"))
            }
            AppError::AIServiceError(_) => {
                log::error!("AI error: {:?}", self);
                HttpResponse::InternalServerError().json(json_error_response("AI service error"))
            }
        }
    }
}

impl From<MultipartError> for AppError {
    fn from(err: MultipartError) -> Self {
        AppError::ValidationError(format!("Multipart error: {}", err))
    }
}

impl From<Utf8Error> for AppError {
    fn from(err: Utf8Error) -> Self {
        AppError::ValidationError(format!("UTF-8 error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::ValidationError(format!("IO error: {}", err))
    }
}

fn json_error_response(message: &str) -> serde_json::Value {
    serde_json::json!({
        "error": message
    })
}