use std::error::Error;

use self::model::{
    OpenAIMessageRequest, OpenAIMessageResponse, OpenAIRun, OpenAIRunResponse, OpenAIThread,
};

pub mod model;

pub struct OpenAI {
    api_key: String,
    assistant_id: String,
    thread_id: String,
    client: reqwest::Client,
    run_id: Option<String>,
}

impl Clone for OpenAI {
    fn clone(&self) -> Self {
        Self {
            api_key: self.api_key.clone(),
            assistant_id: self.assistant_id.clone(),
            thread_id: self.thread_id.clone(),
            client: reqwest::Client::new(),
            run_id: self.run_id.clone(),
        }
    }
}

impl OpenAI {
    pub async fn from(api_key: String, assistant_id: String) -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let thread_id = create_thread(&api_key, &client).await?;
        Ok(Self {
            api_key,
            assistant_id,
            thread_id,
            client,
            run_id: None,
        })
    }

    pub async fn send_message(
        &self,
        message: String,
    ) -> Result<OpenAIMessageResponse, Box<dyn Error>> {
        let message = OpenAIMessageRequest::new(
            "user".to_string(),
            message,
            vec![],
            std::collections::HashMap::new(),
        );

        let message_request = self
            .client
            .post(format!(
                "https://api.openai.com/v1/threads/{}/messages",
                self.thread_id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .json(&message)
            .send()
            .await?;

        let message_response = message_request
            .json::<OpenAIMessageResponse>()
            .await
            .unwrap();

        Ok(message_response)
    }

    pub async fn run_assistant(&mut self) -> Result<model::OpenAIRunResponse, Box<dyn Error>> {
        let run_message = OpenAIRun::new(self.assistant_id.clone(), None);
        let run_assistant = self
            .client
            .post(format!(
                "https://api.openai.com/v1/threads/{}/runs",
                self.thread_id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .json(&run_message)
            .send()
            .await?;

        let run_response = run_assistant.json::<OpenAIRunResponse>().await?;
        self.run_id = Some(run_response.id.clone());
        Ok(run_response)
    }

    pub async fn get_assistant_status(&self) -> Result<model::OpenAIRunResponse, Box<dyn Error>> {
        let status_request = self
            .client
            .get(format!(
                "https://api.openai.com/v1/threads/{}/runs/{}",
                self.thread_id,
                self.run_id.as_ref().unwrap()
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await?;

        let status_response = status_request
            .json::<model::OpenAIRunResponse>()
            .await
            .unwrap();

        Ok(status_response)
    }

    pub async fn get_messages(&self) -> Result<model::OpenAIMessagesResponse, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "https://api.openai.com/v1/threads/{}/messages",
                self.thread_id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await?;

        let response = response
            .json::<model::OpenAIMessagesResponse>()
            .await
            .unwrap();

        Ok(response)
    }

    pub async fn get_assistant_messages(
        &self,
    ) -> Result<Vec<model::OpenAIMessageResponse>, Box<dyn Error>> {
        let response = self
            .client
            .get(format!(
                "https://api.openai.com/v1/threads/{}/messages",
                self.thread_id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await?;

        let response = response
            .json::<model::OpenAIMessagesResponse>()
            .await
            .unwrap();

        let messages = response
            .data
            .iter()
            .filter(|message| message.role == "assistant")
            .map(|message| message.clone())
            .collect::<Vec<model::OpenAIMessageResponse>>();

        Ok(messages)
    }

    pub async fn delete_thread(self) -> Result<reqwest::Response, Box<dyn Error>> {
        let delete_response = self
            .client
            .delete(format!(
                "https://api.openai.com/v1/threads/{}",
                self.thread_id
            ))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("OpenAI-Beta", "assistants=v1")
            .send()
            .await?;

        drop(self);
        Ok(delete_response)
    }
}

async fn create_thread(
    api_key: &String,
    client: &reqwest::Client,
) -> Result<String, Box<dyn Error>> {
    let thread = client
        .post("https://api.openai.com/v1/threads")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("OpenAI-Beta", "assistants=v1")
        .send()
        .await?;

    let thread = thread.json::<OpenAIThread>().await?;

    let thread_id = thread.id;

    Ok(thread_id)
}
