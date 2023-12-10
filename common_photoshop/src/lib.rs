pub mod models;

use axum::http::HeaderMap;
use dotenv::dotenv;

use module_aws::models::ImageJobReply;
use reqwest::header::HeaderValue;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;
use std::env;
use tokio::time::{sleep, Duration};

use crate::models::{
    Color, Input, Job, Mask, Options, Output, PhotoshopClientResponse, PhotoshopJobResponse,
    PhotoshopRequest, Process, Service,
};
use lib_errors::Errors;

use module_aws::utilities::get_aws_client;
use module_aws::utilities::get_output_link;

pub struct Photoshop {
    pub token: String,
    pub id: String,
    pub key: String,
}

impl Photoshop {
    pub async fn new() -> Result<Photoshop, Errors> {
        dotenv().ok();
        let client = reqwest::Client::new();
        let id = env::var("PHOTOSHOP_CLIENT_ID").map_err(|e| Errors::Error(e.to_string()))?;
        let key = env::var("PHOTOSHOP_CLIENT_SECRET").map_err(|e| Errors::Error(e.to_string()))?;
        let mut params = HashMap::new();
        params.insert("grant_type", "client_credentials");
        params.insert("client_id", &id);
        params.insert("client_secret", &key);
        params.insert("scope", "AdobeID,openid");

        let token = client
            .post("https://ims-na1.adobelogin.com/ims/token/v3")
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .form(&params)
            .send()
            .await
            .map_err(|e| Errors::Error(e.to_string()))?
            .json::<PhotoshopClientResponse>()
            .await
            .map_err(|e| Errors::Error(e.to_string()))?
            .access_token
            .ok_or(Errors::Error("failed photoshop client".to_string()))?;

        Ok(Photoshop {
            token: format!("Bearer {}", token),
            id,
            key,
        })
    }

    pub async fn remove_background(
        input_link: &str,
        output_link: &str,
        file_name: &str,
    ) -> Result<ImageJobReply, Errors> {
        // get new access_token
        let photoshop_client = Photoshop::new().await?;
        // reqwest
        let client = reqwest::Client::new();
        // Set headers
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&photoshop_client.token.clone())
                .map_err(|e| Errors::Error(e.to_string()))?,
        );
        headers.insert(
            "x-api-key",
            HeaderValue::from_str(&photoshop_client.id.clone())
                .map_err(|e| Errors::Error(e.to_string()))?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        let request_body = PhotoshopRequest {
            input: Input {
                href: input_link.to_string(),
                storage: "external".to_string(),
            },
            options: Options {
                optimize: "performance".to_string(),
                process: Some(Process { postprocess: true }),
                service: Some(Service {
                    version: "4.0".to_string(),
                }),
            },
            output: Output {
                href: output_link.to_string(),
                storage: "external".to_string(),
                r#type: "image/png".to_string(),
                overwrite: false,
                color: Some(Color {
                    space: "rgb".to_string(),
                }),
                mask: Mask {
                    format: "soft".to_string(),
                },
            },
        };
        // dispatch job to photoshop
        let response = client
            .post("https://image.adobe.io/sensei/cutout")
            .headers(headers.clone())
            .json(&request_body)
            .send()
            .await
            .map_err(|e| Errors::Error(e.to_string()))?
            .json::<PhotoshopJobResponse>()
            .await
            .map_err(|e| Errors::Error(e.to_string()))?;

        let fetch_link = response.links.self_.href;
        let mut status: bool = false;
        let mut link = String::new();
        let mut retry_count = 0;
        let max_retries = 10;
        // check status
        while let false = status {
            retry_count += 1;
            if retry_count >= max_retries {
                // handle the retry limit exceeded scenario
                break;
            }
            let check = client
                .get(fetch_link.clone())
                .headers(headers.clone())
                .send()
                .await
                .map_err(|e| Errors::Error(e.to_string()))?
                .json::<Job>()
                .await
                .map_err(|e| Errors::Error(e.to_string()))?;

            match check.status.as_deref() {
                Some("succeeded") => {
                    status = true;
                    let client = get_aws_client().await?;
                    let bucket =
                        env::var("AWS_BUCKET").map_err(|e| Errors::Error(e.to_string()))?;
                    let mut key =
                        env::var("AWS_IMAGE_KEY").map_err(|e| Errors::Error(e.to_string()))?;
                    key.push_str(file_name);

                    link = get_output_link(&client, &key, &bucket).await?;
                    break;
                }
                Some(_) => {
                    // Wait for 2 seconds before the next iteration
                    sleep(Duration::from_secs(2)).await;
                    continue;
                }
                None => {
                    //update status
                    break;
                }
            }
        }
        //return status with access link
        match link.trim().is_empty() {
            true => Err(Errors::Error("No Response From Photoshop".to_string())),
            _ => Ok(ImageJobReply {
                status: Some(status.to_string()),
                file_name: Some(file_name.to_string()),
                link: Some(link),
            }),
        }
    }
}
