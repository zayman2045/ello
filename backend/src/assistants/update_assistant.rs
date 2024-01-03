use crate::ClientState;
use actix_web::{put, web, HttpResponse, Responder};
use async_openai::types::ModifyAssistantRequestArgs;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct UpdateElloRequest {
    name: String,
    instructions: String,
}

#[derive(Serialize)]
struct UpdateElloResponse {
    assistant_id: String,
}

// Update assistant by id
#[put("/assistants/{assistant_id}")]
async fn update_assistant(
    req: web::Json<UpdateElloRequest>,
    data: web::Data<ClientState>,
    path: web::Path<String>,
) -> impl Responder {
    let client = &data.client;
    let assistant_id = path.into_inner();

    let modify_request = ModifyAssistantRequestArgs::default().name(&req.name).instructions(&req.instructions).build().unwrap(); // TODO: Handle OpenAIError

    let assistant_response = client.assistants().update(&assistant_id, modify_request).await.unwrap(); // TODO: Handle OpenAIError

    let response = UpdateElloResponse {
        assistant_id: assistant_response.id,
    };

    HttpResponse::Ok().json(response)
}