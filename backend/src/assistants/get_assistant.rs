use actix_web::{get, web, HttpResponse, Responder};
use crate::ClientState;
use serde::Serialize;

// Define the structure of the response body for getting an assistant
#[derive(Serialize)]
pub struct AssistantInfo {
    pub id: String,
    pub name: String,
    pub instructions: String,
    pub model: String,
}

// This handler function retrieves a specific assistant
#[get("assistants/{assistant_id}")]
async fn get_assistant(
    data: web::Data<ClientState>, // Shared state
    path: web::Path<String>, // Path parameters
) -> impl Responder {
    // Extract the assistant_id from the path parameters
    let assistant_id = path.into_inner();

    // Get a reference to the client from the shared state
    let client = &data.client;

    // Send the assistant retrieval request and get the response
    let assistant_response = client.assistants().retrieve(&assistant_id).await.unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let assistant_info = AssistantInfo {
        id: assistant_response.id.clone(),
        name: assistant_response.name.clone().unwrap_or("None".to_string()),
        instructions: assistant_response.instructions.clone().unwrap_or("None".to_string()),
        model: assistant_response.model.clone(),
    };

    // Return the response as JSON
    HttpResponse::Ok().json(assistant_info)
}