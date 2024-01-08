use crate::ClientState;
use actix_web::{get, web, HttpResponse, Responder};
use super::get_assistant::AssistantInfo;

// List all assistants
#[get("/assistants")]
async fn list_assistants(data: web::Data<ClientState>) -> impl Responder {
    // Create a query with a limit of 20 assistants
    let query = [("limit", "20")];

    let client = &data.client;
    let assistant_response = client.assistants().list(&query).await.unwrap(); // TODO: Handle OpenAIError
    let assistant_objects = assistant_response.data;

    let assistants = assistant_objects
        .iter()
        .map(|assistant| {
            let assistant_info = AssistantInfo {
                id: assistant.id.clone(),
                name: assistant.name.clone().unwrap_or("None".to_string()),
                instructions: assistant.instructions.clone().unwrap_or("None".to_string()),
                model: assistant.model.clone(),
            };
            assistant_info
        })
        .collect::<Vec<AssistantInfo>>();

    HttpResponse::Ok().json(assistants)
}
