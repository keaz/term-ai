use std::collections::HashMap;

use serde::{Deserialize, Serialize};
// OpenAI Thread Request
#[derive(Debug, Serialize)]
pub struct OpenAIThreadRequest {
    pub messages: Vec<OpenAIMessageRequest>,
}

impl OpenAIThreadRequest {
    pub fn new(messages: Vec<OpenAIMessageRequest>) -> Self {
        Self { messages }
    }
}

#[derive(Debug, Serialize)]
pub struct OpenAIMessageRequest {
    pub role: String,
    pub content: String,
    pub file_ids: Option<Vec<String>>,
    pub metadada: Option<HashMap<String, String>>,
}

impl OpenAIMessageRequest {
    pub fn new(
        role: String,
        content: String,
        file_ids: Option<Vec<String>>,
        metadada: Option<HashMap<String, String>>,
    ) -> Self {
        Self {
            role,
            content,
            file_ids,
            metadada,
        }
    }
}

// OpenAI Thread Response
#[derive(Debug, Deserialize)]
pub struct OpenAIThread {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub metadata: HashMap<String, String>,
}

// OpenAI Modify Thread Request
#[derive(Debug, Serialize)]
pub struct OpenAIModifyThreadRequest {
    pub thread_id: String,
    pub metadada: Option<HashMap<String, String>>,
}

impl OpenAIModifyThreadRequest {
    pub fn new(thread_id: String, metadada: Option<HashMap<String, String>>) -> Self {
        Self {
            thread_id,
            metadada,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct OpenAIMessagesResponse {
    pub object: String,
    pub data: Vec<OpenAIMessageResponse>,
    pub has_more: bool,
    pub first_id: String,
    pub last_id: String,
}

// OPenAI Message Request
#[derive(Debug, Deserialize, Default)]
pub struct OpenAIMessageResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub thread_id: String,
    pub role: String,
    pub content: Vec<OpenAIMessageContent>,
    pub file_ids: Vec<String>,
    pub assistant_id: Option<String>,
    pub run_id: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIMessageContent {
    #[serde(rename = "type")]
    pub content_type: String,
    pub text: OpenAIMessageText,
}

#[derive(Debug, Deserialize)]
pub struct OpenAIMessageText {
    pub value: String,
    pub annotations: Vec<String>,
}

// OpenAI Run Request
#[derive(Debug, Serialize)]
pub struct OpenAIRun {
    pub assistant_id: String,
    pub instructions: Option<String>,
}

impl OpenAIRun {
    pub fn new(assistant_id: String, instructions: Option<String>) -> Self {
        Self {
            assistant_id,
            instructions,
        }
    }
}
