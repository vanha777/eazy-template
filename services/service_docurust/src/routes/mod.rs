use axum::{routing::post, Router};

use crate::handler;

pub fn routes() -> Router {
    Router::new()
        // .route("/", get(handler::welcome::welcome))
        .route("/convert", post(handler::html_to_pdf))
}
