pub mod assistants;
use dotenv::dotenv;
use std::error::Error;

use actix_web::{App, HttpServer, web};
use assistants::{create_assistant, get_assistant};

use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantObject, CreateAssistantRequestArgs, CreateMessageRequestArgs,
        CreateRunRequestArgs, CreateThreadRequestArgs, MessageContent, RunStatus,
    },
    Client,
};

struct ClientState {
    client: Client<OpenAIConfig>,
}


pub async fn run() -> std::io::Result<()> {
    
    // Create a client from the API key in the .env file
    dotenv().ok();
    let client = Client::new();

    // Run HTTP server
    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(ClientState {
            client: client.clone(),
        }))
        .service(create_assistant)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}

// Prompt user to build a new assistant
async fn build_assistant(
    client: &Client<OpenAIConfig>,
    assistants: &mut Vec<AssistantObject>,
) -> Result<(), Box<dyn Error>> {
    // Ask the user for the name of the assistant

    let mut assistant_name = String::new();
    std::io::stdin().read_line(&mut assistant_name)?;
    assistant_name = assistant_name.trim().to_string();

    // Ask the user for the instructions for the assistant

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

    Ok(())
}

// List all existing assistants
fn list_assistants(assistants: &Vec<AssistantObject>) {
    match assistants.len() {
        0 => {}
        _ => {
            assistants.iter().for_each(|a| {
                println!(
                    "\t{}\t{}",
                    a.name
                        .as_ref()
                        .expect("All assistants are created with a name"),
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
        std::io::stdin().read_line(&mut input_name)?;
        input_name = input_name.trim().to_string();

        if input_name.as_str() == "exit" {
            return Ok(());
        } else if !assistant_names.contains(&&input_name) {
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

    loop {
        // Get user input
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        println!("");

        // Break out of the loop if the user enters exit()
        if input.trim() == "exit" {
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
                }
                RunStatus::Failed => {
                    awaiting_response = false;
                }
                RunStatus::Queued => {}
                RunStatus::Cancelling => {}
                RunStatus::Cancelled => {}
                RunStatus::Expired => {}
                RunStatus::RequiresAction => {}
                RunStatus::InProgress => {}
            }
            // Wait for 1 second before checking the status again
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    }

    Ok(())
}
