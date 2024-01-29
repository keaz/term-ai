use std::{collections::HashMap, error::Error};
use std::{thread, time};

use crate::ai::OpenAIMessageRequest;
use crate::ai::{OpenAIMessageResponse, OpenAIRun, OpenAIRunResponse};

mod ai;
mod util;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let api_key = "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"; // Replace with your API key

    let thread = client
        .post("https://api.openai.com/v1/threads")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let thread = thread.json::<ai::OpenAIThread>().await.unwrap();

    let thread_id = thread.id;
    let message = OpenAIMessageRequest::new(
        "user".to_string(),
        "Generate rust code to read user inpunt from terminal until user input 'Q'".to_string(),
        Vec::new(),
        HashMap::new(),
    );

    let message_request = client
        .post(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .json(&message)
        .send()
        .await?;

    let _message_response = message_request
        .json::<OpenAIMessageResponse>()
        .await
        .unwrap();

    let run_message = OpenAIRun::new("asst_hqAXpyDJ4f9f9i3IFv0zne7d".to_string(), None);
    let run_assistant = client
        .post(format!(
            "https://api.openai.com/v1/threads/{}/runs",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .json(&run_message)
        .send()
        .await?;

    let run_response = run_assistant.json::<OpenAIRunResponse>().await.unwrap();

    let mut not_completed = true;

    while not_completed {
        let status_request = client
            .get(format!(
                "https://api.openai.com/v1/threads/{}/runs/{}",
                thread_id, run_response.id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await?;

        let status_response = status_request
            .json::<ai::OpenAIRunResponse>()
            .await
            .unwrap();
        if status_response.status == "in_progress" {
            not_completed = true;
            thread::sleep(time::Duration::from_secs(1));
        } else if status_response.status == "completed" {
            not_completed = false;
        } else {
            println!("{}", status_response.status);
            return Ok(());
        }
    }

    let response = client
        .get(format!(
            "https://api.openai.com/v1/threads/{}/messages",
            thread_id
        ))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let response = response.json::<ai::OpenAIMessagesResponse>().await.unwrap();
    response
        .data
        .iter()
        .filter(|message| message.role == "assistant")
        .for_each(|message| {
            let code = util::extract_code_block(message.content[0].text.value.as_str());
            println!("{:?}", code);
        });

    let delete_response = client
        .delete(format!("https://api.openai.com/v1/threads/{}", thread_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    println!("{}", delete_response.text().await?);
    Ok(())
}
