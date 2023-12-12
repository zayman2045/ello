use dotenv::dotenv;
use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{AssistantObject, CreateAssistantRequestArgs, CreateThreadRequestArgs},
    Client,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    // Load environment variables
    dotenv().ok();

    // create a client
    let client = Client::new();
    let mut assistants: Vec<AssistantObject> = Vec::new();

    loop {
        println!(
            "--- Welcome to Ello. Enter a command or 'help' to see a list of available commands."
        );
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;

        match command.trim() {
            "create" => {
                let assistant = create_assistant(&client).await?;
                assistants.push(assistant);
            }
            "list" => {assistants.iter().for_each(|a| {
                println!("Id: {}\n Name:{:?}", a.id, a.name);
            })},   // list all assistants
            "chat" => (),   // chat with an assistant
            "update" => (), // update an assistant
            "delete" => (), // delete an assistant
            "help" => {println!("TODO: Display help message")},
            "exit" => break,
            _ => continue,
        }
    }

    Ok(())
}

pub async fn create_assistant(
    client: &Client<OpenAIConfig>,
) -> Result<AssistantObject, Box<dyn Error>> {
    //ask the user for the name of the assistant
    println!("--- Enter the name of your new assistant");
    let mut assistant_name = String::new();
    std::io::stdin().read_line(&mut assistant_name)?;

    //ask the user for the instructions for the assistant
    println!("--- Enter the instructions for your new assistant");
    let mut instructions = String::new();
    std::io::stdin().read_line(&mut instructions)?;

    // create an assistant
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&assistant_name)
        .instructions(&instructions)
        .model("gpt-3.5-turbo-1106")
        .build()?;
    let assistant = client.assistants().create(assistant_request).await?;

    Ok(assistant)
}

pub async fn chat(
    client: &Client<OpenAIConfig>,
    assistant_id: &String,
) -> Result<(), Box<dyn Error>> {
    // create a thread
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;


    Ok(())
}
