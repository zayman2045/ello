use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestMessageArgs,
        CreateChatCompletionRequestArgs, Role,
    },
    Client,
};
use dotenv::dotenv;
use std::env;
use std::error::Error;
use std::io::{self, Write};

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

    // Define the conversation history
    let mut conversation_history = Vec::<ChatCompletionRequestMessage>::new();

    // Define the system personality
    let system_initialization_message = ChatCompletionRequestMessageArgs::default()
        .role(Role::System)
        .content("You are a friend named 'Ello' that holds conversation in Spanish at a beginner level. You must come up with interesting answers about yourself when asked questions,including personal information like where you live, your interests, and what you do. Be creative and have fun!.")
        .build()?;

    conversation_history.push(system_initialization_message);

    println!("Welcome to Ello! Start a conversation. Type 'exit' to quit.");

    loop {
        print!("\nYou: ");

        io::stdout().flush().unwrap();

        let mut user_input = String::new();

        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read line");

        user_input = user_input.trim().to_string();

        if user_input == "exit" {
            break;
        } else {
            conversation_history.push(
                ChatCompletionRequestMessageArgs::default()
                    .role(Role::User)
                    .content(user_input.clone())
                    .build()?,
            );
        }

        // Make a request to the API
        let request = CreateChatCompletionRequestArgs::default()
            .max_tokens(512u16)
            .model("gpt-3.5-turbo")
            .messages(conversation_history.clone())
            .build()?;

        let response = client.chat().create(request).await?;

        for choice in response.choices {
            match choice.message.content {
                Some(content) => {
                    conversation_history.push(
                        ChatCompletionRequestMessageArgs::default()
                            .role(Role::Assistant)
                            .content(content.clone())
                            .build()?,
                    );
                    println!("\nEllo: {}", content)
                }
                None => println!("Error: No content in response."),
            }
        }
    }

    println!("Conversation ended.");

    Ok(())
}
