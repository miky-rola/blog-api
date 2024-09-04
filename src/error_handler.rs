use actix_web::{HttpResponse, ResponseError};
use derive_more::Display;
use serde::Serialize;

#[derive(Debug, Display, Serialize)]
pub enum AppError {
    #[display(fmt = "Internal Server Error")]
    InternalServerError,
    #[display(fmt = "Not Found")]
    NotFound,
}

impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::InternalServerError => HttpResponse::InternalServerError().json("Internal Server Error"),
            AppError::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}

impl From<diesel::result::Error> for AppError {
    fn from(error: diesel::result::Error) -> AppError {
        match error {
            diesel::result::Error::NotFound => AppError::NotFound,
            _ => AppError::InternalServerError,
        }
    }
}