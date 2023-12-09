use std::sync::Arc;

use crate::models::error::NeverFailed;
use crate::models::openAi::UserRequest;
use crate::models::ServerState;
use crate::{handler::out_bound::out_call, models::openAi::Response};

use axum::Extension;
//extern crate csvReader;
use axum::{
    extract::Path,
    http::HeaderMap,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Bson};
use serde_json::json;
use tokio_postgres::Config;

pub async fn welcome(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>, //Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    /* */
    //test
    let mongo = state.mongo_client.clone();
    let id = ObjectId::parse_str("650a67768d3b4c5758b208a0").unwrap();
    let res = mongo
        .database("hl7-db")
        .collection::<serde_json::Value>("hl7-config")
        .find_one(doc! {"_id": Bson::ObjectId(id)}, None)
        .await
        .unwrap();
    //end.
    //test
    let conn = state.sql_client.get().await.unwrap();
    let user_id: i32 = 1;
    let rows = conn
        .query("SELECT name FROM users WHERE id = $1", &[&user_id])
        .await
        .unwrap();
    let mut res2 = String::new();
    if let Some(row) = rows.get(0) {
        res2 = row.get::<_, String>(0)
    } else {
        res2 = "User not found".into()
    }
    //end.
    // let res = crate::handler::ai::gpt4::gpt_request::<Response>(&my_body).await?;
    //let res = crate::handler::ai::dale::dale_request(my_body).await?;
    Ok(Json(json!({
        "sql":res2,
        "mongo":res
    })))
}
