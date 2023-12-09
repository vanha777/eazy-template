use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug, Default)]
pub struct VideoRequest {
    pub input: Option<String>,
}

//hey-gen template response
#[derive(Serialize, Deserialize, Debug)]
pub struct GetTemplateResponse {
    pub code: Option<u32>,
    pub data: Data,
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    pub scenes: Vec<Scene>,
    pub template_id: Option<String>,
    pub video_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Scene {
    pub variables: Vec<Variable>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Variable {
    pub name: Option<String>,
    pub properties: Vec<Property>,
    pub r#type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Property {
    pub default: Option<String>,
    pub name: Option<String>,
}

//hey-gen create-video payload
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateVideoRequest {
    pub template_id: Option<String>,
    pub title: Option<String>,
    pub variables: Vec<Variables>,
    pub test: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Variables {
    pub properties: serde_json::Value,
    pub name: Option<String>,
}
/*
#[derive(Serialize, Deserialize, Debug, Clone,Default)]
pub struct Properties {
    pub id: Option<String>,
    pub text: Option<String>,
}*/

#[derive(Serialize, Deserialize, Debug)]
pub struct AttributionsValue {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attributions {
    pub r#type: Option<String>,
    pub properties: Vec<AttributionsValue>,
}
