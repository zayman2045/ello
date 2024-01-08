use actix_web::{post, web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{ClientState, threads::run_thread::run_thread};

// Define the structure of the request body for querying an assistant
#[derive(Deserialize)]
struct QueryElloRequest {
    thread_id: String,
    message: String,
}

// Define the structure of the response body after querying an assistant
#[derive(Serialize)]
struct QueryElloResponse {
    message: String,
}

// This function handler queries an assistant with a message and returns the assistant's response
#[post("/assistants/{assistant_id}")]
async fn query_assistant(
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
    let response_message = run_thread(client, assistant_id, thread_id, request_message).await.unwrap(); // TODO: Handle OpenAIError

    // Construct the response body
    let response = QueryElloResponse {
        message: response_message,
    };

    // Return the response as JSON
    HttpResponse::Ok().json(response)
}
