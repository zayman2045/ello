use actix_web::{post, web, Responder, HttpResponse};
use async_openai::types::CreateThreadRequestArgs;
use serde::Serialize;
use crate::ClientState;

// Define the structure of the response body for creating a thread
#[derive(Serialize)]
struct CreateThreadResponse {
    thread_id: String,
}

// This handler function creates a new thread and returns the thread id
#[post("/threads")]
pub async fn create_thread(
    data: web::Data<ClientState> // Shared state
) -> impl Responder {
    // Create a default thread request
    let thread_request = CreateThreadRequestArgs::default().build().unwrap(); // TODO: Handle OpenAIError
    
    // Get a reference to the client from the shared state
    let client = &data.client;

    // Send the thread creation request and get the response
    let thread = client
        .threads()
        .create(thread_request.clone())
        .await
        .unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let response = CreateThreadResponse {
        thread_id: thread.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
