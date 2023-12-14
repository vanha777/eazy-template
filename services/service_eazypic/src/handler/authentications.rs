use std::sync::Arc;

use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use common_openai::models::UserRequest;
use lib_errors::NeverFailed;
use lib_sharedstate::ServerState;

pub async fn login(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    //login needed to implemented
    let client = state.openai_client.clone();
    let res = common_openai::utilities::dale_request(client, my_body).await?;
    Ok(Json(res))
}

pub async fn signup(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    //login needed to implemented
    let client = state.openai_client.clone();
    let res = common_openai::utilities::dale_request(client, my_body).await?;
    Ok(Json(res))
}
