use chrono::{DateTime, NaiveTime, Utc};
use lib_people::{Class, People};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct UpdateClassStatusRequest {
    pub id: String,
    pub status: Option<bool>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GetClassRequest {
    pub date: String,
    pub class_type: String,
}
