use std::error::Error;

use self::model::{OpenAIMessageRequest, OpenAIMessageResponse, OpenAIRun, OpenAIRunResponse, OpenAIThread};

pub mod model;

const API_KEY: &str = "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";
pub async fn create_thread() -> Result<String, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let thread = client
        .post("https://api.openai.com/v1/threads")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let thread = thread.json::<OpenAIThread>().await.unwrap();

    let thread_id = thread.id;

    Ok(thread_id)
}

pub async fn send_message(
    thread_id: &str,
    message: String,
) -> Result<OpenAIMessageResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();
    let message = OpenAIMessageRequest::new(
        "user".to_string(),
        message,
        vec![],
        std::collections::HashMap::new(),
    );

    let message_request = client
        .post(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .json(&message)
        .send()
        .await?;

    let message_response = message_request.json::<OpenAIMessageResponse>().await.unwrap();

    Ok(message_response)
}


pub async fn run_assistant(
    thread_id: &str,
) -> Result<model::OpenAIRunResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let run_message = OpenAIRun::new("asst_hqAXpyDJ4f9f9i3IFv0zne7d".to_string(), None);
    let run_assistant = client
        .post(format!(
            "https://api.openai.com/v1/threads/{}/runs",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .json(&run_message)
        .send()
        .await?;

    let run_response = run_assistant.json::<OpenAIRunResponse>().await.unwrap();

    Ok(run_response)
}

pub async fn get_assistant_status(
    thread_id: &str,
    run_id: &str,
) -> Result<model::OpenAIRunResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let status_request = client
        .get(format!(
            "https://api.openai.com/v1/threads/{}/runs/{}",
            thread_id, run_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let status_response = status_request
        .json::<model::OpenAIRunResponse>()
        .await
        .unwrap();

    Ok(status_response)
}

pub async fn get_messages(
    thread_id: &str,
) -> Result<model::OpenAIMessagesResponse, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let response = response.json::<model::OpenAIMessagesResponse>().await.unwrap();

    Ok(response)
}

pub async fn get_assistant_messages(
    thread_id: &str,
) -> Result<Vec<model::OpenAIMessageResponse>, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let response = client
        .get(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let response = response.json::<model::OpenAIMessagesResponse>().await.unwrap();

    let messages = response
        .data
        .iter()
        .filter(|message| message.role == "assistant")
        .map(|message| message.clone())
        .collect::<Vec<model::OpenAIMessageResponse>>();

    Ok(messages)
}

pub async fn delete_thread(
    thread_id: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let client = reqwest::Client::new();

    let delete_response = client
        .delete(format!("https://api.openai.com/v1/threads/{}", thread_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", API_KEY))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    Ok(delete_response)
}

