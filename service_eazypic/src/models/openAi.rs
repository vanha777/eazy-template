use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Message {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Request {
    pub model: Option<String>,
    pub messages: Option<Vec<Message>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Choice {
    pub index: Option<i32>,
    pub message: Option<Message>,
    pub finish_reason: Option<String>,
}
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Usage {
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Response {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}
#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ImageGenerationRequest {
    pub prompt: Option<String>,
    pub n: Option<i32>,
    pub size: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UrlData {
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct DaleResponse {
    pub created: Option<i64>,
    pub data: Option<Vec<UrlData>>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct UserRequest {
    pub input: Option<String>,
}
