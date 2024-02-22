//! Handles the listing of messages in a thread.

use crate::{errors::ElloError, ClientState};
use actix_web::{get, web, HttpResponse};
use async_openai::types::{MessageContent, MessageRole};
use serde::Serialize;

/// Response body structure for listing messages.
#[derive(Serialize)]
pub struct MessageInfo {
    pub role: String,
    pub text: String,
}

/// Lists all messages in a specific thread.
#[get("/threads/{thread_id}")]
pub async fn list_messages(
    data: web::Data<ClientState>, 
    path: web::Path<String>,    
) -> Result<HttpResponse, ElloError> {
    // Create a query with a limit of 20 messages
    let query = [("limit", "20")];

    // Extract the thread_id from the path parameters
    let thread_id = path.into_inner();

    // Send the message list request and get the response
    let messages_response = data
        .client
        .threads()
        .messages(&thread_id.clone())
        .list(&query)
        .await
        .map_err(ElloError::from)?;

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
                }
                .to_string(),
                text: match message.content.first() {
                    Some(MessageContent::Text(text)) => text.text.value.clone(),
                    _ => "".to_string(),
                },
            };
            message_info
        })
        .collect::<Vec<MessageInfo>>(); 

    // Return the messages as JSON
    Ok(HttpResponse::Ok().json(messages))
}