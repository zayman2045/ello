//! Main entry point for the application backend.
use ello::run;

/// Starts the backend.
#[tokio::main]
async fn main() {
    run().await.unwrap();
}
