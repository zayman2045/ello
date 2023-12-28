use async_openai::{config::OpenAIConfig, types::CreateThreadRequestArgs, Client};

// Create a new thread and return the thread id
pub async fn create_thread(client: &Client<OpenAIConfig>) -> String {
    let thread_request = CreateThreadRequestArgs::default().build().unwrap(); // TODO: Handle OpenAIError
    
    let thread = client
        .threads()
        .create(thread_request.clone())
        .await
        .unwrap(); // TODO: Handle OpenAIError
    thread.id
}
