//! Handles the deletion of an assistant.

use crate::ClientState;
use actix_web::{delete, web, HttpResponse, Responder};
use serde::Serialize;

/// Response body after deleting an assistant.
#[derive(Serialize)]
pub struct DeleteElloResponse {
    pub assistant_id: String,
}

/// Deletes an assistant and returns its id.
#[delete("/assistants/{assistant_id}")]
pub async fn delete_assistant(
    data: web::Data<ClientState>, // Shared state
    path: web::Path<String>, // Path parameters
) -> impl Responder {
    // Get a reference to the client from the shared state
    let client = &data.client;

    // Extract the assistant_id from the path parameters
    let assistant_id = path.into_inner();

    // Send the assistant deletion request and get the response
    let delete_response = client
        .assistants()
        .delete(&assistant_id)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let response = DeleteElloResponse {
        assistant_id: delete_response.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}