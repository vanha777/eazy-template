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

#[derive(Debug, Serialize, Deserialize)]
struct ClassType {
    id: i32,
    class_type: String,
    days_of_week: String,
    class_references: Option<String>,
    start_time: NaiveTime,
    end_time: NaiveTime,
    create_at: chrono::NaiveDateTime,
    delete_at: Option<chrono::NaiveDateTime>,
    update_at: chrono::NaiveDateTime,
}
