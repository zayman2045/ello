use actix_web::{get, post, web, HttpResponse, Responder};
use async_openai::types::CreateAssistantRequestArgs;

use crate::ClientState;

#[derive(serde::Deserialize)]
struct CreateElloRequest {
    name: String,
    instructions: String,
}

// Create an assistant from CreateElloRequest and respond with assistant id
#[post("/assistants")]
async fn create_assistant(req: web::Json<CreateElloRequest>, data: web::Data<ClientState>) -> impl Responder {
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&req.name)
        .instructions(&req.instructions)
        .model("gpt-3.5-turbo-1106")
        .build().unwrap();
    let assistant = data.client.assistants().create(assistant_request).await.unwrap();
    HttpResponse::Ok().body(format!("Assistant created with id {}", assistant.id))
}

// TODO: Get all assistants
#[get("/assistants")]
async fn get_assistants() -> impl Responder {
    HttpResponse::Ok().body("Assistants found!")
}

// TODO: Get one assistant by id
#[get("/assistants/{id}")]
async fn get_assistant(path: web::Path<u32>) -> impl Responder {
    let path = path.into_inner();
    format!("Assistant with id {} found!", path)
}


// TODO: Send message to assistant and receive response

// TODO: Update assistant by id

// TODO: Delete assistant by id
