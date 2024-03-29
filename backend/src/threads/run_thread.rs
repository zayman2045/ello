//! Handles the execution of a thread using an assistant.

use async_openai::{
    config::OpenAIConfig,
    error::OpenAIError,
    types::{CreateMessageRequestArgs, CreateRunRequestArgs, MessageContent, RunStatus},
    Client,
};

/// Executes a thread using an assistant and a message, and returns the response.
pub async fn run_thread(
    client: &Client<OpenAIConfig>,
    assistant_id: String,
    thread_id: String,
    message: String,
) -> Result<String, OpenAIError> {
    // Create a query with a limit of 1 message
    let query = [("limit", "1")];

    // Create a new message and attach it to the thread
    let message_request = CreateMessageRequestArgs::default()
        .role("user")
        .content(message)
        .build()
        .unwrap();

    let _message = client
        .threads()
        .messages(&thread_id)
        .create(message_request)
        .await
        .unwrap();

    // Create a run
    let run_request = CreateRunRequestArgs::default()
        .assistant_id(assistant_id)
        .build()
        .unwrap();

    let run = client
        .threads()
        .runs(&thread_id)
        .create(run_request)
        .await
        .unwrap();

    // Wait for the run to complete
    let mut awaiting_response = true;
    let mut response_text = "Run failed".to_string();
    while awaiting_response {
        // Retrieve the run
        let run = client
            .threads()
            .runs(&thread_id)
            .retrieve(&run.id)
            .await
            .unwrap();

        // Check the status of the run
        match run.status {
            RunStatus::Completed => {
                awaiting_response = false;

                // Access the response of the run (current list of messages)
                let response = client
                    .threads()
                    .messages(&thread_id)
                    .list(&query)
                    .await
                    .unwrap();

                // Get the latest message id from the response (id of the first message in the list)
                let message_id = response.data.get(0).unwrap().id.clone();

                // Retrieve the message from the thread
                let message = client
                    .threads()
                    .messages(&thread_id)
                    .retrieve(&message_id)
                    .await
                    .unwrap();

                // Get the content from the message
                let content = message.content.get(0).unwrap();

                // Get the text from the content
                response_text = match content {
                    MessageContent::Text(text) => text.text.value.clone(),
                    MessageContent::ImageFile(_) => "Images not supported".to_string(),
                };
            }
            RunStatus::Failed => {
                awaiting_response = false;
            }
            RunStatus::Queued => {}
            RunStatus::Cancelling => {}
            RunStatus::Cancelled => {
                awaiting_response = false;
            }
            RunStatus::Expired => {
                awaiting_response = false;
            }
            RunStatus::RequiresAction => {
                awaiting_response = false;
            }
            RunStatus::InProgress => {}
        }

        // Wait for 1 second before checking the status again
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    Ok(response_text)
}
