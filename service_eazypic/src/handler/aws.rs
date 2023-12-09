use std::env;
use std::sync::Arc;

use crate::models::aws::MediaDownloadResponse;
use crate::models::error::Errors;
use crate::models::{ImageJobReply, RequestInput};
use crate::utilities::aws_s3;
use axum::Json;
use axum::{response::IntoResponse, Extension};
use chrono::{Duration, Utc};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::header::HeaderMap;

use crate::models::{error::NeverFailed, ServerState};

pub async fn get_image_upload_link(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {

    let file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(10) // You can adjust the length
        .map(char::from)
        .collect();
    let client = state.aws_client.clone();
    let mut key = env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
    key.push_str(&file_name);
    key.push_str(".png");
    let bucket = env::var("AWS_BUCKET").map_err(|e| Errors::Error(e.to_string()))?;
    let link = aws_s3::get_input_link(&client, &key, &bucket).await?;

    let res = MediaDownloadResponse {
        item: format!("{}.png", file_name),
        url: link,
        expires: (Utc::now() + Duration::minutes(30)).to_rfc3339(), // expires 30 minutes from now
        created_at: Utc::now().to_rfc3339(),                        // current time
    };
    Ok(Json(res))
}

pub async fn get_image_download_link(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<RequestInput>,
) -> NeverFailed<impl IntoResponse> {
    let file_name = request.id.map(|x| x.clone()).unwrap_or_default();
    let client = state.aws_client.clone();
    let mut key = env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
    key.push_str(&file_name);
    let bucket = env::var("AWS_BUCKET").map_err(|e| Errors::Error(e.to_string()))?;
    let link = aws_s3::get_output_link(&client, &key, &bucket).await?;

    let res = ImageJobReply {
        status: None,
        file_name: Some(file_name),
        link: Some(link.clone()),
    };
    Ok(Json(res))
}
