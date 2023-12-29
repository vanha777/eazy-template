use std::sync::Arc;

use aws_sdk_s3::primitives::ByteStreamError;
use axum::Extension;
use axum::{http::HeaderMap, response::IntoResponse, Json};
use common_openai::models::UserRequest;
use lib_errors::{Errors, NeverFailed};
use lib_people::{People, PeopleMongo};
use lib_sharedstate::ServerState;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{self, bson, Document};

use tokio;

extern crate module_authentication;

pub async fn get_all_student(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
) -> NeverFailed<impl IntoResponse> {
    let sql_client = &state.sql_client;
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<People>("personal_information");

    // Find all documents
    let mut cursor = collection
        .find(None, None) // No filter, get all documents
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
        let my_struct: People =
            bson::from_document(bson_doc).map_err(|e| Errors::Error(e.to_string()))?;
        documents.push(my_struct);
    }

    Ok(Json(documents))
}

pub async fn get_one_student(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<String>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<Document>("personal_information");

    //get people informations
    let people = module_authentication::utilities::get_user(&collection, &request).await?;

    Ok(Json(people))
}

pub async fn update_student(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<People>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<PeopleMongo>("personal_information");

    // convert input to Mongo type
    let object_id = match ObjectId::parse_str(request.id.clone()) {
        Ok(x) => x,
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // Create a query to match the document by index
    let query = mongodb::bson::doc! { "_id": object_id };

    // Define the update to modify the status field
    // let new_document = mongodb::bson::doc! { "$set": { update_field: update_value } };

    // Perform the update
    let _ = collection
        .replace_one(query, request.convert_mongo_people(), None)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(Json(200))
}

pub async fn delete_student(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(request): Json<Vec<String>>,
) -> NeverFailed<impl IntoResponse> {
    let mongo_client = &state.mongo_client;
    // Getting the specific collection
    let database = mongo_client.database("ladomana");
    let collection = database.collection::<PeopleMongo>("personal_information");

    for id in request {
        // convert input to Mongo type
        let object_id = match ObjectId::parse_str(id) {
            Ok(x) => x,
            Err(e) => return Err(Errors::Error(e.to_string())),
        };
        // Create a query to match the document by index
        let query = mongodb::bson::doc! { "_id": object_id };
        // delete people informations
        let _ = collection
            .delete_one(query, None)
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;
    }

    Ok(Json(200))
}
