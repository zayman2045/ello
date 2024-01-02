use actix_web::{get, web, Responder};

use crate::ClientState;

// List all assistants
#[get("/assistants")]
async fn list_assistants(data: web::Data<ClientState>) -> impl Responder {
    // Create a query with a limit of 20 assistants
    let query = [("limit", "20")];

    let client = &data.client;
    let assistant_response = client.assistants().list(&query).await.unwrap(); // TODO: Handle OpenAIError
    let assistants = assistant_response.data;

    // TODO: Serialize into JSON array
    let assistants = assistants
        .iter()
        .map(|assistant| {
            let assistant_id = assistant.id.clone();
            let assistant_name = assistant.name.clone().unwrap_or("None".to_string());
            let assistant_instructions =
                assistant.instructions.clone().unwrap_or("None".to_string());
            let assistant_model = assistant.model.clone();
            let assistant = format!(
                "{{\"id\": \"{}\", \"name\": \"{}\", \"instructions\": \"{}\", \"model\": \"{}\"}}",
                assistant_id, assistant_name, assistant_instructions, assistant_model
            );
            assistant
        })
        .collect::<Vec<String>>()
        .join(",");

    assistants
}
