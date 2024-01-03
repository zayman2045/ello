use crate::ClientState;
use actix_web::{delete, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
struct DeleteElloResponse {
    assistant_id: String,
    deleted: bool,
}

// Delete assistant by id
#[delete("/assistants/{assistant_id}")]
async fn delete_assistant(
    data: web::Data<ClientState>,
    path: web::Path<String>,
) -> impl Responder {
    let client = &data.client;
    let assistant_id = path.into_inner();

    let delete_response = client
        .assistants()
        .delete(&assistant_id)
        .await
        .unwrap(); // TODO: Handle OpenAIError

    let response = DeleteElloResponse {
        assistant_id: delete_response.id,
        deleted: delete_response.deleted,
    };

    HttpResponse::Ok().json(response)
}