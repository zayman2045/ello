use actix_web::{App, HttpServer};

use ello::assistants::{create_assistant, get_assistant};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(create_assistant).service(get_assistant))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
