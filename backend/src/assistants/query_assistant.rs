use actix_web::{post, web, Responder};

use crate::{ClientState, threads::run_thread::run_thread};

#[derive(serde::Deserialize)]
struct QueryElloRequest {
    thread_id: String,
    message: String,
}

// Query the assistant with a message
#[post("/assistants/{assistant_id}")]
async fn query_assistant(
    req: web::Json<QueryElloRequest>,
    data: web::Data<ClientState>,
    path: web::Path<String>,
) -> impl Responder {
    let assistant_id = path.into_inner();
    let message = req.message.clone();
    let thread_id = req.thread_id.clone();
    let client = &data.client;

    let response = run_thread(client, assistant_id, thread_id, message).await.unwrap(); // TODO: Handle OpenAIError

    response
}
