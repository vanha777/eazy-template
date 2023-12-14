pub mod converter;
use std::path::PathBuf;

use axum::{http::HeaderMap, response::IntoResponse, Json};

use base64::encode;
use lib_errors::Errors;
use pdf_gems::merge_pdf::merge_pdf_handler;
use reqwest::header;

use crate::models::{Base64MediaRequest, Base64MediaResponse};

use self::converter::convert_html_to_pdf_base64;

pub async fn html_to_pdf(
    Json(request): Json<Base64MediaRequest>,
) -> Result<impl IntoResponse, Errors> {
    match convert_html_to_pdf_base64(request.html.unwrap_or_default()).await {
        Ok(x) => {
            let base64_pdf = encode(x);

            let res = Base64MediaResponse {
                status_code: Some(200),
                body: Some(base64_pdf),
            };

            // Serialize the response to JSON
            let json_res = serde_json::to_string(&res).map_err(|e| Errors::Error(e.to_string()))?;

            // Calculate the content length
            let content_length = json_res.as_bytes().len();

            // Create custom headersx
            let mut custom_headers = HeaderMap::new();
            custom_headers.insert(
                header::CONTENT_TYPE,
                "application/json; charset=utf-8"
                    .parse()
                    .map_err(|_| Errors::Error("header construct failed".to_string()))?,
            );
            custom_headers.insert(
                header::CONTENT_LENGTH,
                content_length
                    .to_string()
                    .parse()
                    .map_err(|_| Errors::Error("header construct failed".to_string()))?,
            );

            // Note: Content-Length is automatically set by axum for Json responses
            Ok((custom_headers, json_res))
        }
        Err(e) => Err(Errors::Error(e.to_string())),
    }
}

pub async fn merge_pdf() -> Result<impl IntoResponse, Errors> {
    //login needed to implemented
    let vec_pdf_base64 = vec!["your pdf in stream".to_string()];
    let file_path = PathBuf::from("/213");
    if let Err(e) = merge_pdf_handler(vec_pdf_base64, file_path.clone()) {
        eprintln!("Failed to merge PDFs: {}", e);
        return Err(Errors::Error(e.to_string()));
    }
    Ok(Json(200))
}
