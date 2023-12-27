pub mod models;
pub mod utilities;

use std::env;
use std::sync::Arc;

use axum::Json;
use axum::{response::IntoResponse, Extension};
use chrono::{Duration, Utc};
use lib_people::{Class, People};
use models::{Credentials, RegisterCredentials};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::header::HeaderMap;

use lib_errors::{Errors, NeverFailed};
use lib_sharedstate::ServerState;
use mongodb::bson::{doc, Document};
extern crate bcrypt;

pub async fn login(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<Credentials>,
) -> NeverFailed<impl IntoResponse> {
    let sql_client = &state.sql_client;
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<Document>("personal_information");

    // find people uuid
    let people_uuid = crate::utilities::find_login(request, sql_client.clone())
        .await?
        .unwrap_or_default();

    //get people informations
    let people = crate::utilities::get_user(&collection, &people_uuid).await?;

    Ok(Json(people))
}

pub async fn register(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<RegisterCredentials>,
) -> NeverFailed<impl IntoResponse> {
    let sql_client = &state.sql_client;
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<People>("personal_information");
    // check valid credentials
    //
    // create a personal info in mongo
    let uuid = crate::utilities::create_user(&collection, request.people.clone()).await?;
    // create a class information
    let class_collection = database.collection::<Class>("class_information");
    let _ = crate::utilities::create_class(
        &class_collection,
        &uuid,
        request.people.clone(),
        sql_client,
    )
    .await?;
    // insert login detail into sql
    let new_login = Credentials {
        email: request.email,
        password: request.password,
    };
    let _ = crate::utilities::create_login(new_login, &uuid, sql_client.clone()).await?;
    Ok(Json(request.people))
}

pub async fn init(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {
    // Getting the specific collection
    let res = crate::utilities::init(state.sql_client.clone()).await?;

    Ok(Json(res))
}
