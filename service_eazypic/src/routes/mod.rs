use std::sync::Arc;

use crate::handler;
pub mod authentications;
pub mod aws;
pub mod hey_gen;
pub mod openai;
pub mod photoshop;
use axum::{
    http::{
        self,
        header::{AUTHORIZATION, CONTENT_TYPE, LOCATION},
    },
    routing::{get, post},
    Extension, Router,
};
use http::{header, Method, Request, Response};
use serde::{Deserialize, Serialize};

pub fn routes() -> Router {
    Router::new()
        // .route("/", get(handler::welcome::welcome))
        .route("/test", post(handler::welcome::welcome))
        // .route("/test", post(handler::readCsv::csv_handler))
        .merge(openai::routes())
        .merge(hey_gen::routes())
        .merge(authentications::routes())
        .merge(aws::routes())
        .merge(photoshop::routes())
}
