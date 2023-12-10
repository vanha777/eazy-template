use std::sync::Arc;

use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use common_openai::models::UserRequest;
use lib_errors::NeverFailed;
use lib_sharedstate::ServerState;

pub async fn chat_generate(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let client = state.openai_client.clone();
    let res = common_openai::utilities::gpt_request(client, &req.input.unwrap_or_default()).await?;
    Ok(Json(res))
}
