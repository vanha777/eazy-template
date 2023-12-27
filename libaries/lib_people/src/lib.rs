use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct People {
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<String>,
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub class: String, //uuid of class in MySql
    pub number_of_class: Option<u64>,
    pub status: Option<String>,
    pub post_code: Option<String>,
    pub avatar_url: Option<String>,
    pub start_date_1: DateTime<Utc>,
    pub start_date_2: Option<String>,
    pub number_of_month: Option<u64>,
    pub references: Option<String>,
}

// exclusively for service_eazymana
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Class {
    pub personal_information: String,
    pub r#type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Option<bool>,
    pub references: Option<String>,
}
