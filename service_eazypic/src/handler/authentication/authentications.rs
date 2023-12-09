use std::sync::Arc;

use crate::models::error::NeverFailed;
use crate::models::openAi::UserRequest;
use crate::models::ServerState;
use crate::utilities::contentGenerator;
use crate::{handler::out_bound::out_call, models::openAi::Response};
use axum::Extension;
use axum::{
    extract::Path,
    http::HeaderMap,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};

pub async fn login(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    //login needed to implemented
    let client = state.openai_client.clone();
    let res = contentGenerator::dale::dale_request(client, my_body).await?;
    Ok(Json(res))
}

pub async fn signup(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    //login needed to implemented
    let client = state.openai_client.clone();
    let res = contentGenerator::dale::dale_request(client, my_body).await?;
    Ok(Json(res))
}
