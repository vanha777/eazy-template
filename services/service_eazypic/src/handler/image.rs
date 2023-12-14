use std::sync::Arc;

use axum::Extension;
use axum::Json;
use axum::{http::HeaderMap, response::IntoResponse};
use common_openai;
use common_openai::models::UserRequest;
use lib_errors::NeverFailed;
use lib_sharedstate::ServerState;
use module_aws::models::RequestInput;
use module_content;

pub async fn image_url_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_content::image_url_generate(headers, state, Json(request)).await?;
    Ok(res)
}

pub async fn background_remove(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<RequestInput>,
) -> NeverFailed<impl IntoResponse> {
    let remove_background =
        module_content::background_remove(headers, state, Json(request)).await?;
    Ok(remove_background)
}
