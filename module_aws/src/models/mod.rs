use aws_sdk_s3::primitives::ByteStream;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UploadObjectRequest {
    pub image: String, //this is going to be image as base64
    pub item: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DownloadObjectRequest {
    pub item: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MediaUploadRequest {
    pub filename: String, //filename sha256 hash (uuid and timestamp)
    pub variant: String,  //original filename
    pub file_extension: String,
    pub file_type: String,
    pub r#type: String,
    pub region: String,
    pub file_versions: Vec<Version>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MediaDownloadResponse {
    pub item: String,
    pub url: String,
    pub expires: String,
    pub created_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Version {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub bucket: String,
    pub file_size: i32,
    pub url: String,
    pub item: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MediaUploadResponse {
    pub item: String,
    pub url: String,
    pub expires: Option<String>,
    pub created_at: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImageInformation {
    pub item: Option<String>,
    pub url: Option<String>,
    pub expires: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UploadResponse {
    pub key: String,
    pub filename: String,
    pub buckets: Vec<Bucket>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Bucket {
    pub item: String,
    pub bucket_name: String,
}

pub struct StreamFile {
    pub stream: ByteStream,
    pub filename: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct BucketParams {
    pub region: String,
    pub patient_bucket_url: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct AwsHttpResponse {
    pub method: Option<String>,
    pub uri: Option<String>,
    pub headers: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct RequestInput {
    pub data: Option<String>,
    pub id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct ImageJobReply {
    pub status: Option<String>,
    pub file_name: Option<String>,
    pub link: Option<String>,
}
