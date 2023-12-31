pub mod assistants;
pub mod threads;

use actix_web::{web, App, HttpServer};
use assistants::{
    create_assistant::create_assistant, list_assistants::list_assistants,
    query_assistant::query_assistant, delete_assistant::delete_assistant, update_assistant::update_assistant, get_assistant::get_assistant,
};
use async_openai::{config::OpenAIConfig, Client};
use dotenv::dotenv;
use threads::{create_thread::create_thread, list_messages::list_messages};

// Define the shared state for the client
struct ClientState {
    client: Client<OpenAIConfig>,
}

pub async fn run() -> std::io::Result<()> {
    // Create a client from the OPENAI_API_KEY environment variable in the .env file
    dotenv().ok();
    let client = Client::new();

    // Run HTTP server
    HttpServer::new(move || {
        App::new()
            // Share the client state with all handlers
            .app_data(web::Data::new(ClientState {
                client: client.clone(),
            }))
            // Register all handlers
            .service(create_assistant)
            .service(get_assistant)
            .service(list_assistants)
            .service(update_assistant)
            .service(query_assistant)
            .service(delete_assistant)
            .service(create_thread)
            .service(list_messages)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
