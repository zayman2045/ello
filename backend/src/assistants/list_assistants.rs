//! Handles the listing of all assistants.

use super::get_assistant::AssistantInfo;
use crate::{errors::ElloError, ClientState};
use actix_web::{get, web, HttpResponse};

/// Lists all assistants.
#[get("/assistants")]
pub async fn list_assistants(data: web::Data<ClientState>) -> Result<HttpResponse, ElloError> {
    // Create a query with a limit of 20 assistants
    let query = [("limit", "20")];

    // Get a reference to the client from the shared state
    let client = &data.client;

    // Send the assistant list request and get the response
    let assistant_response = client.assistants().list(&query).await.map_err(ElloError::from)?;

    // Extract the assistant objects from the response
    let assistant_objects = assistant_response.data;

    // Map each assistant object to an AssistantInfo struct
    let assistants = assistant_objects
        .iter()
        .map(|assistant| {
            // Create an AssistantInfo struct for each assistant
            let assistant_info = AssistantInfo {
                id: assistant.id.clone(),
                name: assistant.name.clone().unwrap_or("None".to_string()),
                instructions: assistant.instructions.clone().unwrap_or("None".to_string()),
                model: assistant.model.clone(),
            };
            assistant_info
        })
        .collect::<Vec<AssistantInfo>>(); 

    // Return the assistants as JSON
    Ok(HttpResponse::Ok().json(assistants))
}
