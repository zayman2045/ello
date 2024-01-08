use actix_web::{get, web, Responder, HttpResponse};
use async_openai::types::{MessageRole, MessageContent};
use serde::Serialize;
use crate::ClientState;

// Define the structure of the response body for listing messages
#[derive(Serialize)]
struct MessageInfo {
    role: String,
    text: String,
}

// This handler function lists all messages in a specific thread
#[get("/threads/{thread_id}")]
pub async fn list_messages(
    data: web::Data<ClientState>, // Shared state
    path: web::Path<String>, // Path parameters
) -> impl Responder {
    // Create a query with a limit of 20 messages
    let query = [("limit", "20")];

    // Extract the thread_id from the path parameters
    let thread_id = path.into_inner();

    // Send the message list request and get the response
    let messages_response = data.client
        .threads()
        .messages(&thread_id.clone())
        .list(&query)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    // Extract the message objects from the response
    let message_data = messages_response.data;

    // Map each message object to a MessageInfo struct
    let messages = message_data
        .iter()
        .map(|message| {
            // Create a MessageInfo struct for each message
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
        .collect::<Vec<MessageInfo>>(); // Collect the MessageInfo structs into a Vec

    // Return the messages as JSON
    HttpResponse::Ok().json(messages)
}