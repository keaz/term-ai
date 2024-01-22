use std::{collections::HashMap, error::Error};

use crate::ai::{
    OpenAIMessageRequest, OpenAIMessageResponse, OpenAIMessagesResponse, OpenAIRun,
    OpenAIRunResponse,
};

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
    println!("{}", thread.id);

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

    let message_response = message_request
        .json::<OpenAIMessageResponse>()
        .await
        .unwrap();

    println!("{:?}", message_response.content);
    //asst_hqAXpyDJ4f9f9i3IFv0zne7d
    //"https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/runs"
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
    println!("{:?}", run_response.tools);
    //    let status_request = client.get("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/runs/run_6LkoDN57Pz5ithVfR6I2PvfK")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .send().await?;
    //
    //    println!("{}", status_request.text().await?);
    //   let response = client
    //        .get("https://api.openai.com/v1/threads/thread_OYRcF3JXKrQdVdBNDoD54zHE/messages")
    //        .header("Content-Type", "application/json")
    //        .header("Authorization", format!("Bearer {}", api_key))
    //        .header("OpenAI-Beta", "assistants=v1")
    //        .send()
    //        .await?;
    //
    //    let response = response.json::<ai::OpenAIMessagesResponse>().await.unwrap();
    //    //    println!("{:?}", response.data[0].content[0].text.value);
    //    let code = util::extract_code_block(response.data[0].content[0].text.value.as_str());
    //    println!("{:?}", code);

    Ok(())
}
