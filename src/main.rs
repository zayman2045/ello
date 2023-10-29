extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    // Load variables from .env file
    dotenv().ok();

    // Retrieve the API key from environment variables
    match env::var("OPENAI_API_KEY") {
        Ok(api_key) => {
            println!("Successfully read API key: {}", api_key);
            // Your logic here
        }
        Err(_) => {
            println!("Couldn't read OPENAI_API_KEY from environment variables.");
        }
    }
}
