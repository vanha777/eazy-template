use crate::handler;
use axum::{
    http::{
        self,
        header::{AUTHORIZATION, CONTENT_TYPE, LOCATION},
    },
    routing::{get, post},
    Router,
};
use http::{header, Method, Request, Response};
use tower::ServiceBuilder;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

pub fn routes() -> Router {
    Router::new()
        .route("/chat", post(handler::assistant::gpt_chat::chat_generate))
        .route(
            "/image",
            post(handler::generator::image::image_url_generate),
        )
}
