use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStreamError;
use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use chrono::NaiveTime;
use common_openai::models::UserRequest;
use lib_errors::{Errors, NeverFailed};
use lib_people::{Class, ClassMongo, ClassResponse, People};
use lib_sharedstate::ServerState;
use module_authentication::models::{ClassType, ClassTypeSql};
use mongodb::bson::oid::ObjectId;
use mongodb::bson::serde_helpers::serialize_rfc3339_string_as_bson_datetime;
use mongodb::bson::{self, bson, doc, Document};
use mysql_async::prelude::Queryable;
use mysql_async::Row;
use serde_json::from_str;

use crate::models::{GetClassRequest, UpdateClassStatusRequest};
extern crate module_authentication;

pub async fn update_class_status(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<UpdateClassStatusRequest>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<Class>("class_information");
    // convert input to Mongo type
    let object_id = match ObjectId::parse_str(req.id) {
        Ok(x) => x,
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // Create a query to match the document by index
    let query = mongodb::bson::doc! { "_id": object_id };

    // Define the update to modify the status field
    let update = mongodb::bson::doc! { "$set": { "status": req.status } };

    // Perform the update
    match collection.update_one(query, update, None).await {
        Ok(update_result) => Ok(Json(200)),
        Err(e) => return Err(Errors::Error(e.to_string())),
    }
}

pub async fn update_class(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<Class>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<ClassMongo>("class_information");
    // convert input to Mongo type
    let object_id = match ObjectId::parse_str(req.id.clone()) {
        Ok(x) => x,
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // Create a query to match the document by index
    let query = mongodb::bson::doc! { "_id": object_id };

    // Perform the update
    match collection
        .replace_one(query, req.convert_mongo_class(), None)
        .await
    {
        Ok(update_result) => Ok(Json(update_result)),
        Err(e) => return Err(Errors::Error(e.to_string())),
    }
}

pub async fn get_all_class(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<GetClassRequest>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<Class>("class_information");
    // Find all documents
    // Define the filter
    let filter = doc! {
        "type": req.class_type.to_ascii_lowercase(),
        "start_time": req.date
    };
    let mut cursor = collection
        .find(filter, None)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    let mut documents = Vec::new();

    while cursor
        .advance()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?
    {
        let raw_doc = cursor.current();
        let bson_doc =
            bson::from_slice(raw_doc.as_bytes()).map_err(|e| Errors::Error(e.to_string()))?;
        let my_struct: Class =
            bson::from_document(bson_doc).map_err(|e| Errors::Error(e.to_string()))?;
        documents.push(my_struct);
    }
    Ok(Json(documents))
}

pub async fn get_class(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(req): Json<Class>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<Class>("class_information");
    // convert input to Mongo type
    let object_id = match ObjectId::parse_str(req.id.clone()) {
        Ok(x) => x,
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // Create a query to match the document by index
    let query = mongodb::bson::doc! { "_id": object_id };
    let class_info = match collection.find_one(query, None).await {
        Ok(Some(document)) => document,
        Ok(None) => return Err(Errors::Error("Documents not found".to_string())),
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // find student information
    let people_collection = database.collection::<Document>("personal_information");
    //get people informations
    let people =
        module_authentication::utilities::get_user(&people_collection, &req.personal_information)
            .await?;
    let res = ClassResponse {
        student: people.convert_mongo_people(),
        class: class_info,
    };
    Ok(Json(res))
}

pub async fn get_class_type(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {
    let sql_client = &state.sql_client;
    let mut conn = sql_client
        .get_conn()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    //get class type
    let rows: Vec<Row> = conn
        .query("SELECT * FROM class")
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let res: Vec<ClassTypeSql> = rows
        .into_iter()
        .filter_map(|row| {
            // Example of extracting and parsing each field
            let class_type: Option<String> = row.get("class_type");
            let days_of_week_str: Option<String> = row.get("days_of_week");
            let references: Option<String> = row.get("class_references");
            let start_time_str: Option<String> = row.get("start_time");
            let end_time_str: Option<String> = row.get("end_time");

            // if days_of_week_str.is_none() || start_time_str.is_none() || end_time_str.is_none() {
            //     return None; // Skip row if essential fields are missing
            // }

            let days_of_week_str = days_of_week_str
                .unwrap_or_else(|| "".to_string())
                .trim_matches('\'') // Remove leading and trailing single quotes
                .replace("\\\"", "\""); // Replace escaped double quotes with actual double quotes

            let days_of_week: Vec<String> = match from_str(&days_of_week_str) {
                Ok(days) => days,
                Err(_) => return None, // Skip row if JSON parsing fails
            };

            let start_time =
                start_time_str.and_then(|s| NaiveTime::parse_from_str(&s, "%H:%M:%S").ok());

            let end_time =
                end_time_str.and_then(|s| NaiveTime::parse_from_str(&s, "%H:%M:%S").ok());

            // Construct and return ClassType instance
            Some(ClassTypeSql {
                class_type,
                days_of_week,
                references,
                start_time,
                end_time,
            })
        })
        .collect();

    Ok(Json(res))
}
