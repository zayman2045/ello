pub mod assistants;
pub mod threads;

use actix_web::{web, App, HttpServer};
use assistants::{create_assistant::create_assistant, query_assistant::query_assistant, list_assistants::list_assistants};
use dotenv::dotenv;

use async_openai::{config::OpenAIConfig, Client};
use threads::create_thread::create_thread;

struct ClientState {
    client: Client<OpenAIConfig>,
}

pub async fn run() -> std::io::Result<()> {
    // Create a client from the API key in the .env file
    dotenv().ok();
    let client = Client::new();

    // Run HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(ClientState {
                client: client.clone(),
            }))
            .service(create_assistant)
            .service(query_assistant)
            .service(list_assistants)
            .service(create_thread)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
