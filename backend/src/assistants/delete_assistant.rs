use crate::ClientState;
use actix_web::{delete, web, HttpResponse, Responder};
use serde::Serialize;

// Define the structure of the response body for deleting an assistant
#[derive(Serialize)]
struct DeleteElloResponse {
    assistant_id: String,
    deleted: bool,
}

// This handler function deletes a specific assistant
#[delete("/assistants/{assistant_id}")]
async fn delete_assistant(
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
        deleted: delete_response.deleted,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}