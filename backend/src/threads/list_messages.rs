use actix_web::{get, web, Responder, HttpResponse};
use async_openai::types::{MessageRole, MessageContent};
use serde::Serialize;
use crate::ClientState;

#[derive(Serialize)]
struct MessageInfo {
    role: String,
    text: String,
}

// List all messages in a thread
#[get("/threads/{thread_id}")]
pub async fn list_messages(data: web::Data<ClientState>, path: web::Path<String>) -> impl Responder {
    // Create a query with a limit of 20 messages
    let query = [("limit", "20")];

    let thread_id = path.into_inner();

    // Get messages from the thread
    let messages_response = data.client
        .threads()
        .messages(&thread_id.clone())
        .list(&query)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    let message_data = messages_response.data;

    // Extract the role and text from each message
    let messages = message_data
        .iter()
        .map(|message| {
            let message_info = MessageInfo {
                role: match message.role {
                    MessageRole::User => "user",
                    MessageRole::Assistant => "assistant",
                }.to_string(),
                text: match message.content.first().unwrap() {
                    MessageContent::Text(text) => text.text.value.clone(),
                _ => "".to_string(),
                },
            };
            message_info
        })
        .collect::<Vec<MessageInfo>>();

    HttpResponse::Ok().json(messages)
}