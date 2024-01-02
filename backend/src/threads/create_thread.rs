use actix_web::{post, web, Responder, HttpResponse};
use async_openai::types::CreateThreadRequestArgs;
use serde::Serialize;
use crate::ClientState;

#[derive(Serialize)]
struct CreateThreadResponse {
    thread_id: String,
}

// Create a new thread and return the thread id
#[post("/threads")]
pub async fn create_thread(data: web::Data<ClientState>) -> impl Responder {
    let thread_request = CreateThreadRequestArgs::default().build().unwrap(); // TODO: Handle OpenAIError
    
    let thread = data.client
        .threads()
        .create(thread_request.clone())
        .await
        .unwrap(); // TODO: Handle OpenAIError

    let response = CreateThreadResponse {
        thread_id: thread.id,
    };

    HttpResponse::Ok().json(response)
}
