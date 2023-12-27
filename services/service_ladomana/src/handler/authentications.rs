use std::sync::Arc;

use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use common_openai::models::UserRequest;
use lib_errors::NeverFailed;
use lib_sharedstate::ServerState;
use module_authentication::models::{Credentials, RegisterCredentials};

pub async fn login(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<Credentials>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_authentication::login(headers, state, Json(my_body)).await?;
    Ok(res)
}

pub async fn register(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<RegisterCredentials>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_authentication::register(headers, state, Json(my_body)).await?;
    Ok(res)
}

pub async fn init(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {
    let res = module_authentication::init(headers, state).await?;
    Ok(res)
}
