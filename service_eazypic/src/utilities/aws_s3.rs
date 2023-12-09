extern crate aws_config;
extern crate aws_sdk_s3;

use crate::models::error::Errors;
use aws_config::load_defaults;
use aws_config::BehaviorVersion;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client;
use std::env;

use std::time::Duration;

pub async fn get_aws_client() -> Result<Client, Errors> {
    let config = load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    Ok(client)
}

pub async fn list_keys(client: &Client, bucket_name: &str) -> Result<Vec<String>, Errors> {
    // build request
    let req = client.list_objects_v2().prefix("").bucket(bucket_name);
    //execute request
    let res = req.send().await.map_err(|e| Errors::Error(e.to_string()))?;
    let keys = res.contents();

    // collect keys
    let keys = keys
        .iter()
        .filter_map(|o| o.key.as_ref())
        .map(|s| s.to_string())
        .collect::<Vec<_>>();

    Ok(keys)
}

pub fn identify_file_type(data: &[u8]) -> Option<&'static str> {
    match &data[0..4] {
        &[0x89, 0x50, 0x4E, 0x47] => Some("PNG"),
        &[0xFF, 0xD8, ..] => Some("JPEG"),
        &[0x25, 0x50, 0x44, 0x46] => Some("PDF"),
        _ => None,
    }
}
fn get_env_variable(key: &str, default: &str) -> String {
    dotenv::dotenv().ok();
    env::var(key).unwrap_or_else(|_| default.to_string())
}

//this fn is to genereate presigned Url for one hours
#[allow(dead_code)]
pub async fn get_output_link(client: &Client, key: &str, bucket: &str) -> Result<String, Errors> {
    let expiration = Duration::from_secs(3600); // 1 hour
    let req = client.get_object().bucket(bucket).key(key);
    let presigned_request_config =
        PresigningConfig::expires_in(expiration).map_err(|e| Errors::Error(e.to_string()))?;
    let presigned_url = req
        .presigned(presigned_request_config)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let json = presigned_url.uri().to_string();

    Ok(json)
}

pub async fn get_input_link(client: &Client, key: &str, bucket: &str) -> Result<String, Errors> {
    let expiration = Duration::from_secs(1500); // 30 min
    let req = client.put_object().bucket(bucket).key(key);
    let presigned_request_config =
        PresigningConfig::expires_in(expiration).map_err(|e| Errors::Error(e.to_string()))?;
    let presigned_url = req
        .presigned(presigned_request_config)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let json = presigned_url.uri().to_string();

    Ok(json)
}

fn extract_url(response: &str) -> String {
    let start_pattern = "https://";
    let end_pattern = "\", headers:"; // Assuming the URL ends before this pattern

    if let Some(start) = response.find(start_pattern) {
        if let Some(end) = response[start..].find(end_pattern) {
            let url = &response[start..start + end];
            return url.to_string();
        }
    }
    response.to_string()
}
