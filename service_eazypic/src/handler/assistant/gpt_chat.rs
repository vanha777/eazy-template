use std::sync::Arc;

use crate::models::error::NeverFailed;
use crate::models::openAi::{Choice, Message, UserRequest};
use crate::models::ServerState;
use axum::Extension;
//extern crate csvReader;
use crate::utilities::contentGenerator;
use axum::{
    extract::Path,
    http::HeaderMap,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

pub async fn chat_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let client = state.openai_client.clone();
    let res = contentGenerator::gpt4::gpt_request(client, &req.input.unwrap_or_default()).await?;
    Ok(Json(res))
}
