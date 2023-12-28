pub mod assistants;
pub mod threads;

use dotenv::dotenv;
use std::error::Error;
use actix_web::{App, HttpServer, web};
use assistants::create_assistant::create_assistant;

use async_openai::{
    config::OpenAIConfig,
    types::{
        AssistantObject, CreateMessageRequestArgs,
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

async fn run_chat(
    client: &Client<OpenAIConfig>,
    assistants: &Vec<AssistantObject>,
) -> Result<(), Box<dyn Error>> {
    let query = [("limit", "1")]; // limit the responses to 1 message

    // Get the assistant id
    let assistant_id = String::from("Placeholder");

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
