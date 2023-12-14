pub mod models;

use models::{CreateVideoRequest, GetTemplateResponse, VideoRequest};
use reqwest;
use serde_json::{json, Value};
extern crate dotenv;
use common_openai::models::UserRequest;
use lib_errors::Errors;

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

pub async fn video_request(client: &str, request: VideoRequest) -> Result<Value, Errors> {
    let data = json!({
        "background": "#ffffff",
        "clips": [
            {
                "avatar_id": "Daisy-inskirt-20220818",
                "avatar_style": "normal",
                "input_text": request.input,
                "offset": {
                    "x": 0,
                    "y": 0
                },
                "scale": 1,
                "voice_id": "1bd001e7e50f421d891986aad5158bc8"
            }
        ],
       // "ratio": "16:9",
        "ratio": "9:16",
        "test": true,
        "version": "v1alpha"
    });

    let response = reqwest::Client::new()
        .post("https://api.heygen.com/v1/video.generate")
        .header("X-Api-Key", client)
        .header("Content-Type", "application/json")
        .json(&data)
        .send()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let text = response
        .json::<Value>()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(text)
}

pub async fn create_video_by_template(
    client: &str,
    request: CreateVideoRequest,
) -> Result<Value, Errors> {
    let response = reqwest::Client::new()
        .post("https://api.heygen.com/v1/template.generate")
        .header("X-Api-Key", client)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let text = response
        .json::<Value>()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(text)
}

//templateId=39c20fc0bf9c4feda41115c4a9f8fbcc
pub async fn get_template_request(
    client: &str,
    request: UserRequest,
) -> Result<GetTemplateResponse, Errors> {
    let template_id = request.input.unwrap_or_default();
    let url = format!(
        "https://api.heygen.com/v1/template.get?video_id={}",
        template_id
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header("X-Api-Key", client)
        .send()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let res = response
        .json::<GetTemplateResponse>()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(res)
}
