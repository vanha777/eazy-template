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
        .route("/get-upload-link", get(handler::aws::get_image_upload_link))
        .route(
            "/get-download-link",
            post(handler::aws::get_image_download_link),
        )
}
