use chrono::NaiveTime;
use lib_people::People;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct RegisterCredentials {
    pub email: String,
    pub password: String,
    #[serde(flatten)]
    pub people: People,
}

// #[derive(Clone, Serialize, Deserialize, Debug)]
// pub struct NewCredentialsRequest {
//     pub email: String,
//     pub password: String,
//     pub uuid : String
// }

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassType {
    pub days_of_week: Vec<String>,
    pub references: Option<String>,
    pub start_time: Option<NaiveTime>,
    pub end_time: Option<NaiveTime>,
}
