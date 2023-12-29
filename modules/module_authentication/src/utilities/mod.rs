use std::collections::HashSet;

use crate::models::{ClassType, Credentials};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Datelike, Duration, NaiveDateTime, NaiveTime, Utc, Weekday};
use lib_errors::{Errors, NeverFailed};
use lib_people::{Class, People};
use mongodb::{
    bson::{self, doc, oid::ObjectId, Bson, Document},
    Collection,
};
use mysql_async::{prelude::*, Pool, Row};

pub async fn find_login(request: Credentials, client: Pool) -> Result<Option<String>, Errors> {
    let mut conn = client
        .get_conn()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let password: Option<String> = conn
        .exec_first(
            "SELECT password FROM login WHERE email = :email AND delete_at IS NULL",
            params! {"email" => &request.email},
        )
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let valid = verify(request.password, &password.unwrap_or_default())
        .map_err(|e| Errors::Error(e.to_string()))?;

    match valid {
        true => {
            let res: Option<String> = conn
                .exec_first(
                    "SELECT personal_information FROM login WHERE email = :email AND delete_at IS NULL",
                    params! {"email" => &request.email},
                )
                .await
                .map_err(|e| Errors::Error(e.to_string()))?;
            Ok(res)
        }
        _ => Err(Errors::Error("invalid creadentials".to_string())),
    }
}

pub async fn create_login(request: Credentials, uuid: &str, client: Pool) -> Result<u64, Errors> {
    let mut conn = client
        .get_conn()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    let password =
        hash(request.password, DEFAULT_COST).map_err(|e| Errors::Error(e.to_string()))?;
    // Insert the new login record into the database
    let _ = conn
        .exec_drop(
            "INSERT INTO login (email, password, personal_information) VALUES (?, ?, ?)",
            (request.email, password, uuid),
        )
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(200)
}

pub async fn get_user(collection: &Collection<Document>, uuid: &str) -> Result<People, Errors> {
    let object_id = ObjectId::parse_str(uuid).map_err(|e| Errors::Error(e.to_string()))?;
    // Create a query to match the document by index
    let filter = doc! { "_id": &object_id };
    // Find one document matching the query
    let result = match collection.find_one(filter, None).await {
        Ok(Some(document)) => document,
        Ok(None) => return Err(Errors::Error("Documents not found".to_string())),
        Err(e) => return Err(Errors::Error(e.to_string())),
    };
    // Deserialize the BSON document into rust struct
    let res: People = match bson::from_bson(bson::Bson::Document(result)) {
        Ok(value) => value,
        Err(e) => {
            return Err(Errors::Error(e.to_string()));
        }
    };
    Ok(res)
}

pub async fn create_user(
    collection: &Collection<People>,
    people: People,
) -> Result<String, Errors> {
    let id = collection
        .insert_one(&people, None)
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;
    let res = id
        .inserted_id
        .as_object_id()
        .map(|oid| oid.to_hex())
        .unwrap_or("".to_string());
    Ok(res)
}

pub async fn create_class(
    // Assuming `collection` and `Class` are correctly defined elsewhere
    collection: &Collection<Class>,
    people_uuid: &str,
    people_infor: People,
    client: &Pool,
) -> Result<u64, Errors> {
    let class = get_class_type(&people_infor.class, client).await?;

    let class_days: HashSet<Weekday> = class
        .days_of_week
        .iter()
        .map(|day| parse_day_of_week(day))
        .collect();

    let mut current_date = people_infor.start_date_1;
    let number_of_classes = people_infor.number_of_class.unwrap_or_default();
    let mut occurrences = 0;

    while occurrences < number_of_classes {
        if class_days.contains(&current_date.weekday()) {
            let new_start_time = class.start_time;
            let new_end_time = class.end_time;

            let start_time = current_date
                .date()
                .and_hms(
                    chrono::Timelike::hour(&new_start_time.unwrap_or_default()),
                    chrono::Timelike::minute(&new_start_time.unwrap_or_default()),
                    chrono::Timelike::second(&new_start_time.unwrap_or_default()),
                )
                .with_timezone(&Utc);

            let end_time = current_date
                .date()
                .and_hms(
                    chrono::Timelike::hour(&new_end_time.unwrap_or_default()),
                    chrono::Timelike::minute(&new_end_time.unwrap_or_default()),
                    chrono::Timelike::second(&new_end_time.unwrap_or_default()),
                )
                .with_timezone(&Utc);

            let params = Class {
                id: "".to_string(),
                personal_information: people_uuid.to_string(),
                r#type: people_infor.class.to_ascii_lowercase(),
                start_time,
                end_time,
                status: Some(true),
                references: people_infor.references.clone(),
            };

            collection
                .insert_one(&params, None)
                .await
                .map_err(|e| Errors::Error(e.to_string()))?;

            occurrences += 1; // Increment only if a class day is found
        }

        // Move to the next day
        current_date = current_date + Duration::days(1);
    }

    Ok(200)
}

pub async fn get_class_type(class_type: &str, client: &Pool) -> Result<ClassType, Errors> {
    let mut conn = client
        .get_conn()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    // Variables to hold data
    let mut days_of_week: Vec<String> = Vec::new();
    let mut references: Option<String> = None;
    let mut start_time: Option<NaiveTime> = None;
    let mut end_time: Option<NaiveTime> = None;
    // Query to select specific columns where class_type match
    let query_result: Option<(String, Option<String>, String, String)> = conn
    .exec_first("SELECT CAST(days_of_week AS CHAR), CAST(class_references AS CHAR), TIME_FORMAT(start_time, '%H:%i:%s'), TIME_FORMAT(end_time, '%H:%i:%s') FROM class WHERE class_type = :class_type",
        params! {"class_type" => class_type.to_ascii_lowercase()})
        .await.map_err(|e| Errors::Error(e.to_string()))?;

    if let Some((serialized_days, refs, start, end)) = query_result {
        let trimmed_days = serialized_days.trim_matches('\'');
        days_of_week =
            serde_json::from_str(trimmed_days).map_err(|e| Errors::Error(e.to_string()))?;

        references = refs;
        start_time = Some(
            NaiveTime::parse_from_str(&start, "%H:%M:%S")
                .map_err(|e| Errors::Error(e.to_string()))?,
        );
        end_time = Some(
            NaiveTime::parse_from_str(&end, "%H:%M:%S")
                .map_err(|e| Errors::Error(e.to_string()))?,
        );
    }
    let res = ClassType {
        days_of_week: days_of_week,
        references: references,
        start_time,
        end_time,
    };
    Ok(res)
}

pub async fn init(client: Pool) -> Result<u64, Errors> {
    let mut conn = client
        .get_conn()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    // SQL to create the 'login' table
    let create_table_sql = r"
        CREATE TABLE IF NOT EXISTS login (
            id INT AUTO_INCREMENT PRIMARY KEY,
            email VARCHAR(255) NOT NULL UNIQUE,
            password VARCHAR(255) NOT NULL,
            personal_information VARCHAR(36) NOT NULL,
            create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            delete_at TIMESTAMP NULL DEFAULT NULL,
            update_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        );
    ";

    let _ = // Execute the SQL
    conn.query_drop(create_table_sql).await.map_err(|e|Errors::Error(e.to_string()))?;

    // SQL to create the 'class_type' table
    let create_class_type_table_sql = r"
        CREATE TABLE IF NOT EXISTS class (
            id INT AUTO_INCREMENT PRIMARY KEY,
            class_type VARCHAR(255) NOT NULL UNIQUE,
            days_of_week TEXT NOT NULL, 
            class_references TEXT NULL DEFAULT NULL,
            start_time TIME NOT NULL,
            end_time TIME NOT NULL,
            create_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            delete_at TIMESTAMP NULL DEFAULT NULL,
            update_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        );
    ";

    let _ = // Execute the SQL
    conn.query_drop(create_class_type_table_sql).await.map_err(|e|Errors::Error(e.to_string()))?;

    Ok(200)
}

fn parse_day_of_week(day: &str) -> Weekday {
    match day.to_ascii_lowercase().as_str() {
        "monday" => Weekday::Mon,
        "tuesday" => Weekday::Tue,
        "wednesday" => Weekday::Wed,
        "thursday" => Weekday::Thu,
        "friday" => Weekday::Fri,
        "saturday" => Weekday::Sat,
        "sunday" => Weekday::Sun,
        _ => panic!("Invalid day of week: {}", day),
    }
}
