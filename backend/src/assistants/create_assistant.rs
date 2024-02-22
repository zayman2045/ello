//! Handles the creation of a new assistant.

use crate::ClientState;
use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::CreateAssistantRequestArgs;
use serde::{Deserialize, Serialize};

/// Request body for creating an assistant.
#[derive(Deserialize)]
pub struct CreateElloRequest {
    pub name: String,
    pub instructions: String,
    pub model: String,
}

/// Response body after creating an assistant.
#[derive(Serialize)]
pub struct CreateElloResponse {
    pub assistant_id: String,
}

/// Creates a new assistant and returns its id.
#[post("/assistants")]
pub async fn create_assistant(
    req: web::Json<CreateElloRequest>, // Request body
    data: web::Data<ClientState>,      // Shared state
) -> impl Responder {
    // Construct the assistant creation request
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model(&req.model)
        .build()
        .unwrap();

    // Send the assistant creation request and get the response
    let assistant = data
        .client
        .assistants()
        .create(assistant_request)
        .await
        .unwrap();

    // Construct the response body
    let response = CreateElloResponse {
        assistant_id: assistant.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
