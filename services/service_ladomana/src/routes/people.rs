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
        .route("/get-all-student", post(handler::people::get_all_student))
        .route("/get-student", post(handler::people::get_one_student))
        .route("/update-student", post(handler::people::update_student))
        .route("/delete-student", post(handler::people::delete_student))
}
