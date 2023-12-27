use actix_web::{get, post, web, HttpResponse, Responder};

// Get all assistants
#[get("/assistants")]
async fn get_assistants() -> impl Responder {
    HttpResponse::Ok().body("Assistants found!")
}

// Create a new assistant
#[post("/assistants")]
async fn create_assistant() -> impl Responder {
    HttpResponse::Ok().body("Assistant created!")
}

// Get assistant info by id
#[get("/assistants/{id}")]
async fn get_assistant(path: web::Path<u32>) -> impl Responder {
    let path = path.into_inner();
    format!("Assistant with id {} found!", path)
}

// TODO: Update assistant by id

// TODO: Delete assistant by id