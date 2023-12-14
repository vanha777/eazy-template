use std::sync::Arc;

use axum::Json;
use axum::{response::IntoResponse, Extension};
use module_aws::models::RequestInput;
use reqwest::header::HeaderMap;

use lib_errors::NeverFailed;
use lib_sharedstate::ServerState;
use module_aws;

pub async fn get_image_upload_link(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_aws::get_image_upload_link(headers, state).await?;
    Ok(res)
}

pub async fn get_image_download_link(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<RequestInput>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_aws::get_image_download_link(headers, state, Json(request)).await?;
    Ok(res)
}
