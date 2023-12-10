use std::sync::Arc;

use axum::Extension;
use common_openai::models::UserRequest;
use lib_sharedstate::ServerState;
use module_aws::models::RequestInput;
use std::env;

use common_photoshop::Photoshop;
use lib_errors::{Errors, NeverFailed};

use axum::{http::HeaderMap, response::IntoResponse, Json};
use rand::distributions::Alphanumeric;
use rand::Rng;
//extern crate csvReader;

pub async fn chat_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let client = state.openai_client.clone();
    let res = common_openai::utilities::gpt_request(client, &req.input.unwrap_or_default()).await?;
    Ok(Json(res))
}

pub async fn image_url_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let commands = common_openai::utilities::gpt_request(
        state.openai_client.clone(),
        &my_body.input.unwrap_or_default(),
    )
    .await?;

    let res =
        common_openai::utilities::dale3_request(state.openai_client.clone(), &commands).await?;
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
    let input_link =
        module_aws::utilities::get_output_link(&state.aws_client.clone(), &key, &bucket).await?;
    // generate input link
    let mut file_name2: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10) // You can adjust the length
        .map(char::from)
        .collect();
    file_name2.push_str(".png");
    let mut key2 = env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
    key2.push_str(&file_name2);
    let output_link =
        module_aws::utilities::get_input_link(&state.aws_client.clone(), &key2, &bucket).await?;
    // call photoshop api removal background
    let remove_background =
        Photoshop::remove_background(&input_link, &output_link, &file_name2).await?;

    Ok(Json(remove_background))
}
