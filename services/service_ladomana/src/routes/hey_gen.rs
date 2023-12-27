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
            "/create-video",
            post(handler::hey_gen::hey_gen_create_video),
        )
        .route(
            "/{uuid}/create-video",
            post(handler::hey_gen::hey_gen_create_video_by_template),
        )
        .route("/get-video", post(handler::hey_gen::hey_gen_get_video))
        .route(
            "/get-template",
            post(handler::hey_gen::hey_gen_get_template_by_id),
        )
}
