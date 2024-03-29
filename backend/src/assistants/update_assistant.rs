//! Handles the editing of an assistant.

use crate::{errors::ElloError, ClientState};
use actix_web::{put, web, HttpResponse};
use async_openai::types::ModifyAssistantRequestArgs;
use serde::{Deserialize, Serialize};

/// Request body for editing an assistant.
#[derive(Deserialize)]
pub struct UpdateElloRequest {
    pub name: String,
    pub instructions: String,
    pub model: String,
}

/// Response body after editing an assistant.
#[derive(Serialize)]
pub struct UpdateElloResponse {
    pub assistant_id: String,
}

/// Edits an existing assistant and returns its id.
#[put("/assistants/{assistant_id}")]
pub async fn update_assistant(
    req: web::Json<UpdateElloRequest>,
    data: web::Data<ClientState>,
    path: web::Path<String>,
) -> Result<HttpResponse, ElloError> {
    // Get a reference to the client from the shared state
    let client = &data.client;

    // Extract the assistant_id from the path parameters
    let assistant_id = path.into_inner();

    // Construct the assistant modification request
    let modify_request = ModifyAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model(&req.model)
        .build()
        .map_err(ElloError::from)?;

    // Send the assistant modification request and get the response
    let assistant_response = client
        .assistants()
        .update(&assistant_id, modify_request)
        .await
        .map_err(ElloError::from)?;

    // Construct the response body
    let response = UpdateElloResponse {
        assistant_id: assistant_response.id,
    };

    // Return the response as JSON
    Ok(HttpResponse::Ok().json(response))
}
