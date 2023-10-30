use async_openai::{
    config::OpenAIConfig,
    types::{ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs, Role},
    Client,
};
use dotenv::dotenv;
use std::env;
use std::error::Error;

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

    let config = OpenAIConfig::new().with_api_key(api_key);
    // .with_org_id("the-continental");

    let client = Client::with_config(config);

    let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(512u16)
        .model("gpt-3.5-turbo")
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content("You are a friend that holds conversation in Spanish, adjusting the difficulty level depending on the flow of the conversation.")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content("¿Hola como estás?")
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

    Ok(())
}
