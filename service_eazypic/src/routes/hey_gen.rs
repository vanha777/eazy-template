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
        .route(
            "/create-video",
            post(handler::generator::hey_gen::hey_gen_create_video),
        )
        .route(
            "/{uuid}/create-video",
            post(handler::generator::hey_gen::hey_gen_create_video_by_template),
        )
        .route(
            "/get-video",
            post(handler::generator::hey_gen::hey_gen_get_video),
        )
        .route(
            "/get-template",
            post(handler::generator::hey_gen::hey_gen_get_template_by_id),
        )
}
