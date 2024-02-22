//! Custom error handling for the backend.

use actix_web::{error::ResponseError, HttpResponse};
use async_openai::error::OpenAIError;
use thiserror::Error;

/// Custom error type for wrapping errors.
#[derive(Error, Debug)]
pub enum ElloError {
    /// An error occurred with the OpenAI API.
    #[error("An OpenAI error occurred")]
    OpenAI {
        #[from]
        source: OpenAIError,
    }
}

impl ResponseError for ElloError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ElloError::OpenAI  { source } => {
                match source {
                    _ => HttpResponse::InternalServerError().json("An error occurred")
                }
            }
        }
    }
}