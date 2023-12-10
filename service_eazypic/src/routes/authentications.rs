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
        .route("/signin", post(handler::authentications::login))
        .route("/signup", post(handler::authentications::signup))
}
