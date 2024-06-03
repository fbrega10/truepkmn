use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;
use serde::{Deserialize, Serialize};

const ERROR_CODE_01: &str = "ERROR01";
const ERROR_CODE_02: &str = "ERROR02";
const ERROR_CODE_03: &str = "ERROR03";
const SERVICE_UNAVAILABLE: &str = "Service unavailable";
const POKEMON_NOT_FOUND: &str = "Pokemon not found!";
const POKEMON_TIMEOUT: &str = "Connection timeout: could not connect to the server";
#[derive(Debug, Display, PartialEq)]
#[allow(dead_code)]
pub enum PokeError {
    ServiceUnavailable,
    NotFound,
    TimeoutError,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    errors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomError {
    error_code: String,
    error_message: String,
}

impl CustomError {
    fn new(error_code: &str, error_message: &str) -> CustomError {
        CustomError {
            error_code: error_code.to_string(),
            error_message: error_message.to_string(),
        }
    }
}

impl From<PokeError> for CustomError {
    fn from(value: PokeError) -> Self {
        match value {
            PokeError::ServiceUnavailable => CustomError::new(ERROR_CODE_01, SERVICE_UNAVAILABLE),
            PokeError::NotFound => CustomError::new(ERROR_CODE_02, POKEMON_NOT_FOUND),
            PokeError::TimeoutError => CustomError::new(ERROR_CODE_03, POKEMON_TIMEOUT),
        }
    }
}

//PokeError needs to be shifted to a ResponseError in case of failure
//found out these specific cases, in case of error connecting to the server a ServiceUnavailable Exception is thrown
impl ResponseError for PokeError {
    fn error_response(&self) -> HttpResponse {
        match self {
            PokeError::TimeoutError => {
                HttpResponse::BadRequest().json(CustomError::new(ERROR_CODE_01, POKEMON_TIMEOUT))
            }
            PokeError::NotFound => {
                HttpResponse::NotFound().json(CustomError::new(ERROR_CODE_02, POKEMON_NOT_FOUND))
            }
            PokeError::ServiceUnavailable => HttpResponse::ServiceUnavailable()
                .json(CustomError::new(ERROR_CODE_03, SERVICE_UNAVAILABLE)),
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
