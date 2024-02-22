//! Handles the creation of a new thread.

use crate::ClientState;
use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::CreateThreadRequestArgs;
use serde::Serialize;

/// Response body structure for creating a thread.
#[derive(Serialize)]
pub struct CreateThreadResponse {
    pub thread_id: String,
}

/// Creates a new thread and returns the thread id.
#[post("/threads")]
pub async fn create_thread(data: web::Data<ClientState>, // Shared state
) -> impl Responder {
    // Create a default thread request
    let thread_request = CreateThreadRequestArgs::default().build().unwrap();

    // Get a reference to the client from the shared state
    let client = &data.client;

    // Send the thread creation request and get the response
    let thread = client
        .threads()
        .create(thread_request.clone())
        .await
        .unwrap();

    // Construct the response body
    let response = CreateThreadResponse {
        thread_id: thread.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
