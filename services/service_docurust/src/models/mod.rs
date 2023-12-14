use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Base64MediaRequest {
    pub html: Option<String>,
    pub multi_html: Option<Vec<String>>,
}
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Base64MediaResponse {
    #[serde(rename = "statusCode")]
    pub status_code: Option<i32>,
    pub body: Option<String>,
}
