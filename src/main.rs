use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::io;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load variables from .env file
    dotenv().ok();

    // Retrieve the API key from environment variables
    let api_key = match env::var("OPENAI_API_KEY") {
        Ok(api_key) => api_key,
        Err(_) => {
            println!("Couldn't read OPENAI_API_KEY from environment variables.");
            return Ok(());
        }
    };

    // Create a client with the API key
    let config = OpenAIConfig::new().with_api_key(api_key);
    let client = Client::with_config(config);

    let mut user_history = Vec::<String>::new();
    let mut assistant_history = Vec::<String>::new();

    println!("Welcome to the GPT-3 chatbot demo. Start a conversation. Type 'exit' to quit.");

    loop {

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input = user_input.trim().to_string();

        if user_input == "exit" {
            break;
        }

        // Make a request to the API
        let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            // Define the system personality
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a friend that holds conversation in Spanish, adjusting the difficulty level depending on the flow of the conversation.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(user_input)
                .build()?,
        ])
        .build()?;

        let response = client.chat().create(request).await?;

        for choice in response.choices {
            match choice.message.content {
                Some(content) => println!("{}", content),
                None => println!("No content"),
            }
        }
    }

    Ok(())
}
