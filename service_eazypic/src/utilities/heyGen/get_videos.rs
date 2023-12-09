use reqwest;
use serde_json::{json, Value};
extern crate dotenv;
use crate::models;
use crate::models::openAi::UserRequest;
use models::error::Errors;

pub async fn get_video_request(client: &str, request: UserRequest) -> Result<Value, Errors> {
    let video_id = request.input.unwrap_or_default();
    let url = format!(
        "https://api.heygen.com/v1/video_status.get?video_id={}",
        video_id
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header("X-Api-Key", client)
        .send()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let res = response
        .json::<Value>()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(res)
}
