//! Main server functionality for the application.

pub mod assistants;
pub mod threads;
pub mod errors;

use actix_web::{web, App, HttpServer};
use assistants::{
    create_assistant::create_assistant, list_assistants::list_assistants,
    query_assistant::query_assistant, delete_assistant::delete_assistant, update_assistant::update_assistant, get_assistant::get_assistant,
};
use async_openai::{config::OpenAIConfig, Client};
use dotenv::dotenv;
use threads::{create_thread::create_thread, list_messages::list_messages};

/// Shared client state.
struct ClientState {
    client: Client<OpenAIConfig>,
}

/// Starts the HTTP server and shares the client state with all handlers.
pub async fn run() -> std::io::Result<()> {
    dotenv().ok();
    let client = Client::new();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ClientState {
                client: client.clone(),
            }))
            .service(create_assistant)
            .service(get_assistant)
            .service(list_assistants)
            .service(update_assistant)
            .service(query_assistant)
            .service(delete_assistant)
            .service(create_thread)
            .service(list_messages)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}