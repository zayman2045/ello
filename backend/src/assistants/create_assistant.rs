use crate::ClientState;
use actix_web::{post, web, HttpResponse, Responder};
use async_openai::types::CreateAssistantRequestArgs;

#[derive(serde::Deserialize)]
struct CreateElloRequest {
    name: String,
    instructions: String,
}

// Create an assistant from CreateElloRequest and respond with assistant id
#[post("/assistants")]
async fn create_assistant(
    req: web::Json<CreateElloRequest>,
    data: web::Data<ClientState>,
) -> impl Responder {
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model("gpt-3.5-turbo-1106")
        .build()
        .unwrap(); // TODO: Handle OpenAIError

    let assistant = data
        .client
        .assistants()
        .create(assistant_request)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    HttpResponse::Ok().body(format!("Assistant created with id {}", assistant.id))
}