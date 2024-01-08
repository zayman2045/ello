use crate::ClientState;
use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::CreateAssistantRequestArgs;
use serde::{Deserialize, Serialize};

// Define the structure of the request body for creating an assistant
#[derive(Deserialize)]
struct CreateElloRequest {
    name: String,
    instructions: String,
}

// Define the structure of the response body after creating an assistant
#[derive(Serialize)]
struct CreateElloResponse {
    assistant_id: String,
}

// This function handler creates an assistant and returns its id
#[post("/assistants")]
async fn create_assistant(
    req: web::Json<CreateElloRequest>, // Request body
    data: web::Data<ClientState>, // Shared state
) -> impl Responder {
    // Construct the assistant creation request
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model("gpt-3.5-turbo-1106")
        .build()
        .unwrap(); // TODO: Handle OpenAIError

    // Send the assistant creation request and get the response
    let assistant = data
        .client
        .assistants()
        .create(assistant_request)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let response = CreateElloResponse {
        assistant_id: assistant.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
