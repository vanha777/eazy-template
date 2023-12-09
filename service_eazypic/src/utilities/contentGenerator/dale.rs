use async_openai::types::{CreateImageRequest, ImagesResponse};
use polars::error::map_err;
use reqwest;
use serde_json::{json, Value};
extern crate dotenv;
use crate::models;
use crate::models::openAi::{DaleResponse, ImageGenerationRequest, UserRequest};
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateImageRequestArgs,
        ImageSize, ResponseFormat, Role,
    },
    Client,
};
use dotenv::dotenv;
use models::error::Errors;
use std::env;

pub async fn dale_request(
    client: Client<OpenAIConfig>,
    request: UserRequest,
) -> Result<Value, Errors> {
    let request = CreateImageRequestArgs::default()
        .prompt(request.input.unwrap_or_default())
        .n(1)
        // .response_format(ResponseFormat::B64Json)
        .response_format(ResponseFormat::Url)
        .size(ImageSize::S256x256)
        .user("async-openai")
        .build()
        .map_err(|e| Errors::Error(e.to_string()))?;

    let response = client
        .images()
        .create(request)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    Ok(json!(response))
}

pub async fn dale3_request(
    client: Client<OpenAIConfig>,
    request: &str,
) -> Result<ImagesResponse, Errors> {
    let request = CreateImageRequest {
        prompt: request.to_string(),
        model: Some(async_openai::types::ImageModel::DallE3),
        n: Some(1),
        quality: Some(async_openai::types::ImageQuality::Standard), // to test, HD when in production
        response_format: Some(ResponseFormat::B64Json),             // can be base64
        size: Some(ImageSize::S1024x1024), //just for testing, inscrease in production
        style: Some(async_openai::types::ImageStyle::Vivid),
        user: None, //not needed for now
    };

    let response = client
        .images()
        .create(request)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    Ok(response)
}
