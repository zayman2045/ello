use actix_web::{get, web, HttpResponse, Responder};
use crate::ClientState;
use serde::Serialize;

#[derive(Serialize)]
pub struct AssistantInfo {
    pub id: String,
    pub name: String,
    pub instructions: String,
    pub model: String,
}

#[get("assistants/{assistant_id}")]
async fn get_assistant(data: web::Data<ClientState>, path: web::Path<String>) -> impl Responder {
    let assistant_id = path.into_inner();
    let client = &data.client;

    let assistant_response = client.assistants().retrieve(&assistant_id).await.unwrap(); // TODO: Handle OpenAIError
    
    let assistant_info = AssistantInfo {
        id: assistant_response.id.clone(),
        name: assistant_response.name.clone().unwrap_or("None".to_string()),
        instructions: assistant_response.instructions.clone().unwrap_or("None".to_string()),
        model: assistant_response.model.clone(),
    };
    HttpResponse::Ok().json(assistant_info)
}