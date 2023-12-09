use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use super::openAi;

pub type NeverFailed<T> = core::result::Result<T, Errors>;

#[derive(Error, Debug)]
pub enum Errors {
    #[error("Error: {0}")]
    Error(String),
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        (StatusCode::IM_A_TEAPOT, self.to_string()).into_response()
    }
}
