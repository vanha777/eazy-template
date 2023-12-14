use crate::handler;
use axum::{routing::post, Router};

pub fn routes() -> Router {
    Router::new().route("/make-character", post(handler::image::background_remove))
}
