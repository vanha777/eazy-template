use crate::models;
use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage, ChatCompletionRequestUserMessage,
        ChatCompletionRequestUserMessageContent, CreateChatCompletionRequest, Role,
    },
    Client,
};

pub async fn gpt_request(
    client: Client<OpenAIConfig>,
    input: &str,
) -> Result<String, models::error::Errors> {
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
        .map_err(|e| models::error::Errors::Error(e.to_string()))?;
    Ok(res.choices[0].message.content.clone().unwrap_or_default())
}
