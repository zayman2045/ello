//! Custom error handling for the backend.

use actix_web::{error::ResponseError, HttpResponse};
use async_openai::error::OpenAIError;
use std::fmt;

/// Custom error type for wrapping OpenAI errors.
pub struct ElloError {
    pub inner: OpenAIError,
}

impl fmt::Debug for ElloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.inner)
    }
}

impl fmt::Display for ElloError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<OpenAIError> for ElloError {
    fn from(error: OpenAIError) -> Self {
        ElloError { inner: error }
    }
}

impl ResponseError for ElloError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError().json("An error occurred")
    }
}