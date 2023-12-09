use std::env;
use std::sync::Arc;

use crate::models::error::{Errors, NeverFailed};
use crate::models::openAi::UserRequest;
use crate::models::{RequestInput, ServerState};
use crate::utilities::photoshop::Photoshop;
use crate::utilities::{aws_s3, contentGenerator};
use axum::Extension;
use axum::{
    extract::Path,
    http::HeaderMap,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use rand::distributions::Alphanumeric;
use rand::Rng;

pub async fn image_url_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let commands = contentGenerator::gpt4::gpt_request(
        state.openai_client.clone(),
        &my_body.input.unwrap_or_default(),
    )
    .await?;

    let res = contentGenerator::dale::dale3_request(state.openai_client.clone(), &commands).await?;
    Ok(Json(res))
}

pub async fn background_remove(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<RequestInput>,
) -> NeverFailed<impl IntoResponse> {
    // receive files name
    let file_name = request.id.unwrap_or_default();
    let mut key = env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
    key.push_str(&file_name);
    // generate download presigned link
    let bucket = env::var("AWS_BUCKET").map_err(|e| Errors::Error(e.to_string()))?;
    let input_link = aws_s3::get_output_link(&state.aws_client.clone(), &key, &bucket).await?;
    // generate input link
    let mut file_name2: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10) // You can adjust the length
        .map(char::from)
        .collect();
    file_name2.push_str(".png");
    let mut key2 = env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
    key2.push_str(&file_name2);
    let output_link = aws_s3::get_input_link(&state.aws_client.clone(), &key2, &bucket).await?;
    // call photoshop api removal background
    let remove_background =
        Photoshop::remove_background(&input_link, &output_link, &file_name2).await?;

    Ok(Json(remove_background))
}
