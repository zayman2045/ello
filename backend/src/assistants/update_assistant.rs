use crate::ClientState;
use actix_web::{put, web, HttpResponse, Responder};
use async_openai::types::ModifyAssistantRequestArgs;
use serde::{Serialize, Deserialize};

// Define the structure of the request body for updating an assistant
#[derive(Deserialize)]
struct UpdateElloRequest {
    name: String,
    instructions: String,
    model: String,
}

// Define the structure of the response body after updating an assistant
#[derive(Serialize)]
struct UpdateElloResponse {
    assistant_id: String,
}

// This handler function updates an assistant and returns its id
#[put("/assistants/{assistant_id}")]
async fn update_assistant(
    req: web::Json<UpdateElloRequest>, // Request body
    data: web::Data<ClientState>, // Shared state
    path: web::Path<String>, // Path parameters
) -> impl Responder {
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
        .unwrap(); // TODO: Handle OpenAIError

    // Send the assistant modification request and get the response
    let assistant_response = client
        .assistants()
        .update(&assistant_id, modify_request)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let response = UpdateElloResponse {
        assistant_id: assistant_response.id,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}