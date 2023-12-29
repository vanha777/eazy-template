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

pub fn routes() -> Router {
    Router::new()
        .route(
            "/update-class-status",
            post(handler::class::update_class_status),
        )
        .route("/update-class", post(handler::class::update_class))
        .route("/get-all-class", post(handler::class::get_all_class))
        .route("/get-class", post(handler::class::get_class))
}
