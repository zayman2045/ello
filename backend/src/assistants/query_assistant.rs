//! Handles the querying of an assistant.

use crate::{threads::run_thread::run_thread, ClientState};
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

/// Request body for querying an assistant.
#[derive(Deserialize)]
pub struct QueryElloRequest {
    pub thread_id: String,
    pub message: String,
}

/// Response body after querying an assistant.
#[derive(Serialize)]
pub struct QueryElloResponse {
    pub message: String,
}

/// Queries an assistant and returns its response message and thread_id.
#[post("/assistants/{assistant_id}")]
pub async fn query_assistant(
    req: web::Json<QueryElloRequest>,
    data: web::Data<ClientState>,
    path: web::Path<String>,
) -> impl Responder {
    // Extract the assistant_id from the path parameters
    let assistant_id = path.into_inner();

    // Extract the message and thread_id from the request body
    let request_message = req.message.clone();
    let thread_id = req.thread_id.clone();

    // Get a reference to the client from the shared state
    let client = &data.client;

    // Run the thread and get the response message
    let response_message = run_thread(client, assistant_id, thread_id, request_message)
        .await
        .unwrap();

    // Construct the response body
    let response = QueryElloResponse {
        message: response_message,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
