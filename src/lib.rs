use dotenv::dotenv;
use std::error::Error;

use async_openai::{
    config::OpenAIConfig,
    types::{AssistantObject, CreateAssistantRequestArgs, CreateThreadRequestArgs},
    Client,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Create a client
    let client = Client::new();
    let mut assistants: Vec<AssistantObject> = Vec::new();

    // Main terminal input loop
    loop {
        println!(
            "--- Welcome to Ello. Enter a command or 'help' to see a list of available commands."
        );
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;

        match command.trim() {
            "build" => build_assistant(&client, &mut assistants).await?,
            "list" => list_assistants(&assistants),
            "chat" => {
                if assistants.is_empty() {
                    println!("--- You currently have 0 assistants. Use 'build' to create a new assistant.");
                    continue;
                };
                println!(
                    "--- You currently have {} assistants:",
                    assistants.iter().count()
                );
            } // chat with an assistant
            "inspect" => (), // inspect an assistant
            "update" => (),  // update an assistant
            "delete" => (),  // delete an assistant
            "help" => {
                println!("TODO: Display help message")
            }
            "exit" => break,
            _ => continue,
        }
    }

    Ok(())
}

// Prompt user to build a new assistant
async fn build_assistant(
    client: &Client<OpenAIConfig>,
    assistants: &mut Vec<AssistantObject>,
) -> Result<(), Box<dyn Error>> {
    //ask the user for the name of the assistant
    println!("--- Enter the name of your new assistant");
    let mut assistant_name = String::new();
    std::io::stdin().read_line(&mut assistant_name)?;
    assistant_name = assistant_name.trim().to_string();

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

    assistants.push(assistant);

    Ok(())
}

// List all existing assistants
fn list_assistants(assistants: &Vec<AssistantObject>) {
    let assistant_count = assistants.len();
    match assistant_count {
        0 => {
            println!("--- You currently have 0 assistants. Use 'create' to build a new assistant.")
        }
        _ => {
            println!("--- You currently have {} assistants:", assistant_count);
            assistants.iter().for_each(|a| {
                println!(
                    "Name:{}",
                    a.name
                        .as_ref()
                        .expect("All assistants are created with a name")
                );
            });
        }
    }
}

async fn start_chat(
    client: &Client<OpenAIConfig>,
    assistant_id: &String,
) -> Result<(), Box<dyn Error>> {
    // Create a new thread
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;

    Ok(())
}
