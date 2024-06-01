use actix_web::http::StatusCode;
use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum PokeError {
    ServiceUnavailable(String),
    NotFound(String),
    TimeoutError(String),
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

//PokeError needs to be shifted to a ResponseError in case of failure
//found out these specific cases, in case of error connecting to the server a ServiceUnavailable Exception is thrown
impl ResponseError for PokeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PokeError::ServiceUnavailable(error) => HttpResponse::BadRequest().finish(),
            PokeError::NotFound(error) => HttpResponse::NotFound().finish(),
            PokeError::ServiceUnavailable(error) => HttpResponse::ServiceUnavailable().finish(),
            _ => HttpResponse::new(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}
impl From<&String> for ErrorResponse {
    fn from(error: &String) -> Self {
        ErrorResponse {
            errors: vec![error.into()],
        }
    }
}

/// Utility to transform a vector of strings into an ErrorResponse
impl From<Vec<String>> for ErrorResponse {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}
