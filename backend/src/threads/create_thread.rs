use actix_web::{post, web};
use async_openai::types::CreateThreadRequestArgs;

use crate::ClientState;

// Create a new thread and return the thread id
#[post("/threads")]
pub async fn create_thread(data: web::Data<ClientState>) -> String {
    let thread_request = CreateThreadRequestArgs::default().build().unwrap(); // TODO: Handle OpenAIError
    
    let thread = data.client
        .threads()
        .create(thread_request.clone())
        .await
        .unwrap(); // TODO: Handle OpenAIError
    
    thread.id
}
