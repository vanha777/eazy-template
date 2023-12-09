use reqwest;
extern crate dotenv;
use crate::models;
use crate::models::heygen::GetTemplateResponse;
use crate::models::openAi::UserRequest;
use models::error::Errors;
//templateId=39c20fc0bf9c4feda41115c4a9f8fbcc
pub async fn get_template_request(
    client: &str,
    request: UserRequest,
) -> Result<GetTemplateResponse, Errors> {
    let template_id = request.input.unwrap_or_default();
    let url = format!(
        "https://api.heygen.com/v1/template.get?video_id={}",
        template_id
    );

    let response = reqwest::Client::new()
        .get(&url)
        .header("X-Api-Key", client)
        .send()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    let res = response
        .json::<GetTemplateResponse>()
        .await
        .map_err(|e| Errors::Error(e.to_string()))?;

    Ok(res)
}
