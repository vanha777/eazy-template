use chrono::{DateTime, NaiveDateTime, Utc, NaiveTime};
use mongodb::bson::oid::ObjectId;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct People {
    #[serde(alias = "_id", deserialize_with = "deserialize_objectid_to_string")]
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<String>,
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub class: String,
    pub number_of_class: Option<u64>,
    pub status: Option<String>,
    pub post_code: Option<String>,
    pub avatar_url: Option<String>,
    pub start_date_1: DateTime<Utc>,
    pub start_date_2: Option<String>,
    pub number_of_month: Option<u64>,
    pub references: Option<String>,
    pub number_of_class_per_week: Option<Vec<String>>,
    pub number_of_minute_per_class: Option<u64>,
    pub class_detail: Vec<ClassPerWeek>
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassPerWeek {
    pub class_type: String,
    pub day : String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
}

impl People {
    pub fn convert_mongo_people(&self) -> PeopleMongo {
        PeopleMongo {
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            name: self.name.clone(),
            email: self.email.clone(),
            mobile: self.mobile.clone(),
            address: self.address.clone(),
            date_of_birth: self.date_of_birth.clone(),
            suburb: self.suburb.clone(),
            city: self.city.clone(),
            class: self.class.clone(),
            number_of_class: self.number_of_class,
            status: self.status.clone(),
            post_code: self.post_code.clone(),
            avatar_url: self.avatar_url.clone(),
            start_date_1: self.start_date_1.clone(),
            start_date_2: self.start_date_2.clone(),
            number_of_month: self.number_of_month,
            references: self.references.clone(),
            number_of_class_per_week: self.number_of_class_per_week.clone(),
            number_of_minute_per_class: self.number_of_minute_per_class.clone(),
            class_detail: self.class_detail.clone()
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct PeopleMongo {
    pub first_name: String,
    pub last_name: String,
    pub name: String,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub address: Option<String>,
    pub date_of_birth: Option<String>,
    pub suburb: Option<String>,
    pub city: Option<String>,
    pub class: String,
    pub number_of_class: Option<u64>,
    pub status: Option<String>,
    pub post_code: Option<String>,
    pub avatar_url: Option<String>,
    pub start_date_1: DateTime<Utc>,
    pub start_date_2: Option<String>,
    pub number_of_month: Option<u64>,
    pub references: Option<String>,
    pub number_of_class_per_week: Option<Vec<String>>,
    pub number_of_minute_per_class: Option<u64>,
    pub class_detail: Vec<ClassPerWeek>
}

fn deserialize_objectid_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let object_id_result = ObjectId::deserialize(deserializer);
    match object_id_result {
        Ok(oid) => Ok(oid.to_hex()),
        Err(_) => Ok(String::new()), // Return empty string on error
    }
}

// exclusively for service_eazymana
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Class {
    #[serde(alias = "_id", deserialize_with = "deserialize_objectid_to_string")]
    pub id: String,
    pub personal_information: String,
    pub r#type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Option<bool>,
    pub references: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassMongo {
    pub personal_information: String,
    pub r#type: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Option<bool>,
    pub references: Option<String>,
}

impl Class {
    pub fn convert_mongo_class(&self) -> ClassMongo {
        ClassMongo {
            personal_information: self.personal_information.clone(),
            r#type: self.r#type.clone(),
            start_time: self.start_time.clone(),
            end_time: self.end_time.clone(),
            status: self.status.clone(),
            references: self.references.clone(),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ClassResponse {
    #[serde(flatten)]
    pub class: Class,
    #[serde(flatten)]
    pub student: PeopleMongo,
}
