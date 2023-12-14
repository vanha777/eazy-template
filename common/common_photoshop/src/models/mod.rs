use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PhotoshopClientResponse {
    pub access_token: Option<String>,
    pub expires_in: Option<u64>,
    pub token_type: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PhotoshopRequest {
    pub input: Input,
    pub options: Options,
    pub output: Output,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Input {
    pub href: String,
    pub storage: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Options {
    pub optimize: String,
    pub process: Option<Process>,
    pub service: Option<Service>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Process {
    pub postprocess: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Service {
    pub version: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Output {
    pub href: String,
    pub storage: String,
    pub r#type: String,
    pub overwrite: bool,
    pub color: Option<Color>,
    pub mask: Mask,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Color {
    pub space: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Mask {
    pub format: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct PhotoshopJobResponse {
    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_: SelfLink,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct SelfLink {
    pub href: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Job {
    pub job_id: Option<String>,
    pub status: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
    pub input: Option<String>,
    pub options: Option<Options>,
    pub metadata: Option<Metadata>,
    #[serde(rename = "_links")]
    pub links: Option<Links>,
    pub output: Option<Output>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Metadata {
    pub service: Service,
    pub model: Option<Model>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct Model {
    pub classification: String,
    pub universal: String,
}
