use reqwest;
use serde_json::{json, Value};
use std::error::Error;

pub async fn out_call<T: serde::Serialize, U: serde::de::DeserializeOwned>(
    files: T,
    url: String,
) -> Result<U, Box<dyn Error>> {
    //let url = "https://api.apis.guru/v2/list.json";
    let file = serde_json::to_value(files).unwrap();
    println!("{}", file);

    let response = reqwest::get(url).await?;

    Ok(response.json().await?)
}
