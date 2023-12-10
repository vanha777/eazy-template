use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest,
        CreateImageRequestArgs, ImageSize, ResponseFormat, Role,
    },
    Client,
};

use crate::models;
use crate::models::{DaleResponse, ImageGenerationRequest, UserRequest};
use async_openai::types::{CreateImageRequest, ImagesResponse};
use reqwest;
use serde_json::{json, Value};

use dotenv::dotenv;
use lib_errors::Errors;
use std::env;

pub async fn gpt_request(client: Client<OpenAIConfig>, input: &str) -> Result<String, Errors> {
    let chat_config:Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestMessage::System(ChatCompletionRequestSystemMessage{
             content: Some("You are a helpful assistant for transforming user ideas into AI text commands used to generate images".to_string()), role: Role::System,name:None
            }),
        ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage{
             content: Some(ChatCompletionRequestUserMessageContent::Text("A Facebook interface resembling Windows, featuring a cute, cartoonish, half-human robot presenter in its natural habitat, peeling and tearing off Facebook pages to connect with you.".to_string())), 
             role: Role::User,name:None
            }),
        ChatCompletionRequestMessage::Assistant(ChatCompletionRequestAssistantMessage{
             content: Some("Illustrate a scene wherein a cartoonish and cute half-robot-half-human female character is attractively posed inside a social media-themed window, designed with modern blue and white symbols. She's shown in her natural environment, a futuristic sphere filled with high-tech components. In a friendly gesture, she is extending her arm through the window frame, removing pages from the digital window to reach towards the viewer, as though welcoming us into her ultramodern world.".to_string()), 
             role: Role::Assistant,
              tool_calls:None,
               function_call: None ,
               name:None
            }),
            ChatCompletionRequestMessage::User(ChatCompletionRequestUserMessage{
                content: Some(ChatCompletionRequestUserMessageContent::Text(input.to_string())),
                role: Role::User,name:None
               })
        ];

    let request = CreateChatCompletionRequest {
        messages: chat_config,
        model: "gpt-4".to_string(),
        frequency_penalty: Some(2.0),
        logit_bias: None,
        max_tokens: None,
        n: Some(1),                  // output message
        presence_penalty: Some(2.0), //creativity
        response_format: None,
        seed: None,
        stop: None,
        stream: None,
        temperature: Some(1.0),
        top_p: None,
        tools: None,
        tool_choice: None,
        user: None,
        function_call: None,
        functions: None,
    };

    let res = client
        .chat()
        .create(request)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    Ok(res.choices[0].message.content.clone().unwrap_or_default())
}

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
