use core::fmt;

use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;
use serde_derive::Serialize;
use strum::ParseError;

#[derive(Debug)]
pub enum AppErrorType {
    NotFoundError,
    ReqwestError,
    InvalidTimeFormat,
    InvalidRequestParameter,
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError {
            cause: Some(err.to_string()),
            message: Some("API request failed".into()),
            error_type: AppErrorType::ReqwestError,
        }
    }
}

impl From<ParseError> for AppError {
    fn from(value: ParseError) -> Self {
        AppError {
            cause: Some(value.to_string()),
            message: Some("The requested item was not found".into()),
            error_type: AppErrorType::NotFoundError,
        }
    }
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: _,
                message: None,
                error_type: AppErrorType::NotFoundError,
            } => "The requested item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
}

#[derive(Serialize)]
struct AppErrorResponse {
    error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
            AppErrorType::ReqwestError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::InvalidTimeFormat => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::InvalidRequestParameter => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}
