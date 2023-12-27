use colored::*;
use dotenv::dotenv;
use std::error::Error;

pub mod assistants;

use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantObject, CreateAssistantRequestArgs, CreateMessageRequestArgs,
        CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent, RunStatus,
    },
    Client,
};

pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    // Create a client
    let client = Client::new();
    let mut assistants: Vec<AssistantObject> = Vec::new();

    // Welcome message
    println!(
        "{}\n",
        "--- Welcome to Ello. Enter a command or use 'help' to see a list of available commands."
            .cyan()
    );

    // Main terminal input loop
    loop {
        let mut command = String::new();
        std::io::stdin().read_line(&mut command)?;

        match command.trim() {
            "exit" => break,
            "build" => build_assistant(&client, &mut assistants).await?,
            "list" => list_assistants(&assistants),
            "chat" => run_chat(&client, &assistants).await?,
            "update" => (), // update an assistant
            "delete" => (), // delete an assistant
            "help" => display_help(),
            _ => display_help(),
        }
    }

    Ok(())
}

// Prompt user to build a new assistant
async fn build_assistant(
    client: &Client<OpenAIConfig>,
    assistants: &mut Vec<AssistantObject>,
) -> Result<(), Box<dyn Error>> {
    // Ask the user for the name of the assistant
    println!("{}", "--- Enter the name of your new assistant:\n".cyan());
    let mut assistant_name = String::new();
    std::io::stdin().read_line(&mut assistant_name)?;
    assistant_name = assistant_name.trim().to_string();

    // Ask the user for the instructions for the assistant
    println!(
        "{}",
        "--- Enter the instructions for your new assistant:\n".cyan()
    );
    let mut instructions = String::new();
    std::io::stdin().read_line(&mut instructions)?;

    // Create an assistant
    let assistant_request = CreateAssistantRequestArgs::default()
        .name(&assistant_name)
        .instructions(&instructions)
        .model("gpt-3.5-turbo-1106")
        .build()?;

    let assistant = client.assistants().create(assistant_request).await?;

    assistants.push(assistant);

    println!(
        "{}",
        "--- Assistant created. Use 'chat' to begin a new chat with a created assistant.\n".cyan()
    );

    Ok(())
}

// List all existing assistants
fn list_assistants(assistants: &Vec<AssistantObject>) {
    match assistants.len() {
        0 => {
            println!("{}", "--- You have no existing assistants. Use 'create' to build a new assistant.\n".yellow());
        }
        _ => {
            println!("{}", "--- Your assistants:\n".cyan());
            assistants.iter().for_each(|a| {
                println!(
                    "\t{}\t{}",
                    a.name
                        .as_ref()
                        .expect("All assistants are created with a name").green(),
                    a.instructions
                        .as_ref()
                        .expect("All assistants are given instructions.")
                );
            });
        }
    }
}

async fn run_chat(
    client: &Client<OpenAIConfig>,
    assistants: &Vec<AssistantObject>,
) -> Result<(), Box<dyn Error>> {
    let query = [("limit", "1")]; // limit the responses to 1 message

    list_assistants(assistants);
    if assistants.is_empty() {
        return Ok(());
    }

    let assistant_names: Vec<&String> = assistants
        .iter()
        .map(|a| {
            a.name
                .as_ref()
                .expect("All assistants are created with a name")
        })
        .collect();

    let assistant_name;
    loop {
        // Prompt the user for the assistant name
        let mut input_name = String::new();
        println!("{}", "--- Which assistant would you like to chat with:\n".cyan());
        std::io::stdin().read_line(&mut input_name)?;
        input_name = input_name.trim().to_string();

        if input_name.as_str() == "exit" {
            println!("{}", "--- Exiting chat.\n".cyan());
            return Ok(());
        } else if !assistant_names.contains(&&input_name) {
            println!("{}", "--- That name does not match an existing assistant.".yellow());
            println!("{}", "--- Enter the name of an existing assistant or use 'exit' to return to the main menu.\n".yellow());
            continue;
        }
        assistant_name = input_name;
        break;
    }

    // Get the assistant id
    let assistant_id = &assistants
        .iter()
        .find(|a| a.name.as_ref().unwrap() == &assistant_name)
        .expect("At this point the assistant is known to exist")
        .id;

    // Create a new thread
    let thread_request = CreateThreadRequestArgs::default().build()?;
    let thread = client.threads().create(thread_request.clone()).await?;

    // Begin a chat
    println!("{}", "--- New chat started. Use 'exit' to end the chat.\n".cyan());

    loop {
        println!("{}", "--- You:".purple());
        // Get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("");

        // Break out of the loop if the user enters exit()
        if input.trim() == "exit" {
            println!("{}", "--- Exiting chat.\n".cyan());
            break;
        }
        
        // Create a message and attach it to the current thread
        let message_request = CreateMessageRequestArgs::default()
            .role("user")
            .content(input.clone())
            .build()?;

        let _message = client
            .threads()
            .messages(&thread.id)
            .create(message_request)
            .await?;

        // Create a run for the assistant using the current thread
        let run_request = CreateRunRequestArgs::default()
            .assistant_id(assistant_id)
            .build()?;
        let run = client
            .threads()
            .runs(&thread.id)
            .create(run_request)
            .await?;

        // Wait for the run to complete
        let mut awaiting_response = true;
        while awaiting_response {
            // Retrieve the run
            let run = client.threads().runs(&thread.id).retrieve(&run.id).await?;

            // Check the status of the run
            match run.status {
                RunStatus::Completed => {
                    awaiting_response = false;

                    // Access the response of the run (current list of messages)
                    let response = client.threads().messages(&thread.id).list(&query).await?;

                    // Get the latest message id from the response (id of the first message in the list)
                    let message_id = response.data.get(0).unwrap().id.clone();

                    // Retrieve the message from the thread
                    let message = client
                        .threads()
                        .messages(&thread.id)
                        .retrieve(&message_id)
                        .await?;

                    // Get the content from the message
                    let content = message.content.get(0).unwrap();

                    // Get the text from the content
                    let text = match content {
                        MessageContent::Text(text) => text.text.value.clone(),
                        MessageContent::ImageFile(_) => {
                            panic!("imaged are not supported in the terminal")
                        }
                    };

                    // Print the text
                    println!("\n{}\n{}\n", "--- Response:".green(), text);
                }
                RunStatus::Failed => {
                    awaiting_response = false;
                    println!("{} {:#?}", "--- Run Failed:".red(), run);
                }
                RunStatus::Queued => {
                    println!("{}", "--- Run Queued".cyan());
                }
                RunStatus::Cancelling => {
                    println!("{}", "--- Run Cancelling".cyan());
                }
                RunStatus::Cancelled => {
                    println!("{}", "--- Run Cancelled".cyan());
                }
                RunStatus::Expired => {
                    println!("{}", "--- Run Expired".cyan());
                }
                RunStatus::RequiresAction => {
                    println!("{}", "--- Run Requires Action".cyan());
                }
                RunStatus::InProgress => {
                    println!("{}", "--- Waiting for response...".cyan());
                }
            }
            // Wait for 1 second before checking the status again
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}

fn display_help() {
    println!("{}", "--- Commands:".cyan());
    println!("\t{}\t{}", "build".blue(), "Create a new assistant.");
    println!("\t{}\t{}", "list".blue(), "List all the created assistants.");
    println!("\t{}\t{}", "chat".blue(), "Start a chat with the assistants.");
    println!("\t{}\t{}", "help".blue(), "Display the available commands.");
    println!("\t{}\t{}", "exit".blue(), "Exit the application.");
    println!("");
}
