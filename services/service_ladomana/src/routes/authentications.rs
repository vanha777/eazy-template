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
        .route("/init", post(handler::authentications::init))
        .route("/login", post(handler::authentications::login))
        .route("/register", post(handler::authentications::register))
}
