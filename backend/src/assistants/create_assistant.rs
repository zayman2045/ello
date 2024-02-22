//! Handles the creation of a new assistant.

use crate::{errors::ElloError, ClientState};
use actix_web::{post, web, HttpResponse};
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
    req: web::Json<CreateElloRequest>,
    data: web::Data<ClientState>,
) -> Result<HttpResponse, ElloError> {
    // Construct the assistant creation request
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model(&req.model)
        .build().map_err(ElloError::from)?;

    // Send the assistant creation request and get the response
    let assistant = data
        .client
        .assistants()
        .create(assistant_request)
        .await
        .map_err(ElloError::from)?;

    // Construct the response body
    let response = CreateElloResponse {
        assistant_id: assistant.id,
    };

    // Return the response as JSON
    Ok(HttpResponse::Ok().json(response))
}
