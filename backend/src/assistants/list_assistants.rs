use actix_web::{get, HttpResponse, Responder};

// TODO: List all assistants
#[get("/assistants")]
async fn list_assistants() -> impl Responder {
    HttpResponse::Ok().body("Assistants found!")
}