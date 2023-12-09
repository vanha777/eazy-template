use std::collections::HashMap;
use std::sync::Arc;

use crate::models::error::NeverFailed;
use crate::models::heygen::{
    Attributions, AttributionsValue, CreateVideoRequest, Variables, VideoRequest,
};
use crate::models::openAi::UserRequest;
use crate::models::ServerState;
use crate::utilities::heyGen;
use crate::utilities::heyGen::create_video::create_video_by_template;
use axum::Extension;
use axum::{
    extract::Path,
    http::HeaderMap,
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use serde_json::json;
//const VIDEO_COMPONENT:Vec<String> = vec!["text".to_string(),"image".to_string(),"photar".to_string(),"video".to_string(),"avatar".to_string()];
//const COMPONENT_VALUE:Vec<String> = vec!["fit".to_string(),"id".to_string(),"voice_id".to_string(),"link".to_string(),"text".to_string(),"play_style".to_string()];

pub async fn hey_gen_create_video(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<VideoRequest>,
) -> NeverFailed<impl IntoResponse> {
    let res = heyGen::create_video::video_request(&state.heygen_client, my_body).await?;
    Ok(Json(res))
}

pub async fn hey_gen_create_video_by_template(
    headers: HeaderMap,
    Path(uuid): Path<String>,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<VideoRequest>,
) -> NeverFailed<impl IntoResponse> {
    //create_video payload
    let mut video_variables: Vec<Variables> = Vec::new();
    let req = CreateVideoRequest {
        template_id: my_body.input.clone(),
        title: Some("this is your video tile".to_string()),
        variables: video_variables,
        test: Some(true), // true for testing purposes
    };

    let res =
        heyGen::create_video::create_video_by_template(&state.heygen_client, req.clone()).await?;

    Ok(Json(res))
}

pub async fn hey_gen_get_video(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let client = state.heygen_client.clone();
    let res = heyGen::get_videos::get_video_request(&client, my_body).await?;
    Ok(Json(res))
}

//fn that return a template with prefiled variable by user profile and a test video url
pub async fn hey_gen_get_template_by_id(
    headers: HeaderMap,
    state: Extension<Arc<ServerState>>,
    Json(my_body): Json<UserRequest>,
) -> NeverFailed<impl IntoResponse> {
    let client = state.heygen_client.clone();
    let res = heyGen::get_template::get_template_request(&client, my_body).await?;

    //get variable from template
    let vec_scene: Vec<HashMap<String, HashMap<String, String>>> = res
        .data
        .scenes
        .iter()
        .map(|x| {
            let data = &x.variables;
            let map: HashMap<String, HashMap<String, String>> = data
                .iter()
                .map(|variable| {
                    let inner_map: HashMap<String, String> = variable
                        .properties
                        .iter()
                        .map(|x| {
                            (
                                x.name.clone().unwrap_or_default(),
                                x.default.clone().unwrap_or_default(),
                            )
                        })
                        .collect();
                    (variable.name.clone().unwrap_or_default(), inner_map)
                })
                .collect();
            map
        })
        .collect();

    //example of user profile settings
    let avatar_att: Vec<AttributionsValue> = serde_json::from_value(json!(
     [
            {
            "name":"id",
            "value":"Mark-blueshirt-20220601"
            },
            {
                "name":"text",
                "value":"Hello mine dear friends"
            },
            {
                "name":"voice_id",
                "value":"331f1cc78737475b946079cb3d2f5ffc"
            }
        ]
    ))
    .unwrap();
    let user_avatar: Attributions = Attributions {
        r#type: Some("avatar_1".to_owned()),
        properties: avatar_att,
    };

    let number_scene = vec_scene.len();

    //loop through vec of scene
    let map_user_profile: Vec<HashMap<String, HashMap<String, String>>> = vec_scene
        .iter()
        .map(|x| {
            //loop thorugh each scene
            let scene: HashMap<String, HashMap<String, String>> = x
                .iter()
                .map(|(k, y)| {
                    //loop thourgh each variable in scene
                    let mut value_hashmap: HashMap<String, String> = HashMap::new();
                    let value_unique_name_split = k.split('_').next().unwrap_or("");

                    //find avatar and logo
                    match value_unique_name_split {
                        "photar" => { // | "avatar"
                            //mapping user define avatar
                            for (key, value) in y {
                                match key.as_str() {
                                    //looping thourgh properties of avatar and replace id,
                                    "id" => value_hashmap.insert(
                                       key.to_string(),
                                       //value.to_string()
                                        "0f32e8513d3248849aacc33958442d6d".to_string(),
                                    ),
                                    // if not id return as it is
                                    _ => value_hashmap.insert(key.to_string(), value.to_string()),
                                };
                            }
                        }
                        _ => {
                            //return the rest properties as it is
                            match k.as_str() {
                                "image_0" => {
                                    // assume image_0 is logo by default
                                    for (key, value) in y {
                                        match key.as_str() {
                                            //looping thourgh properties of avatar and replace id,
                                            "link" => value_hashmap.insert(
                                                key.to_string(),
                                                "https://files.movio.la/pacific/resource/image/admin/eadf7d67aaab4041a983776ab369e543.png?Expires=1697977263&Signature=AYOLFgFX4VpALpeMR6KyDlNaXMBQzrZTJvYLBm2JXTAlFrF8vhnVNU5tuulQzPE~BJQV0q1LX0dpAA40MRG7dLWKrwmDwZutLyP7dWpzSuy08-HqL7Ol203ccq9BBuShOChDEQRIObZumtt3Vc-oWK7gCSZilTNyBJqvCfGhGvpkecdLNryXS447SXXX~8fniIADjhUIwZxtPxzPuwq3oCPJlKdHKBV2nx7U9gCzMrOAHJNQ8mN-Q0Azsgk788AycMXG3dsniLcX~3lyaXTU9KDKqBe5ZgrN02JFGJvNTLrx0TnaqmYf23Aw4chBnWytfmVOWlFWHoUKHr1iXP64Ww__&Key-Pair-Id=K49TZTO9GZI6K".to_string(),
                                            ),
                                            // if not id return as it is
                                            _ => value_hashmap
                                                .insert(key.to_string(), value.to_string()),
                                        };
                                    }
                                }
                                _ => {
                                    for (key, value) in y {
                                        value_hashmap.insert(key.to_string(), value.to_string());
                                    }
                                }
                            }
                        }
                    };
                    (k.clone(), value_hashmap)
                })
                .collect();
            return scene;
        })
        .collect();

    //insert changed-values into payload to construct new test video
    let video_variables: Vec<Variables> = map_user_profile
        .iter()
        .flat_map(|x| {
            // let mut variables:Variables = Variables::default();
            x.iter().map(|(unique_name, variables)| Variables {
                properties: serde_json::to_value(variables).unwrap_or_default(),
                name: Some(unique_name.to_owned()),
            })
        })
        .collect();

    //create test video
    let payload = CreateVideoRequest {
        template_id: res.data.template_id.clone(),
        title: Some("this is your sample template".to_string()),
        variables: video_variables,
        test: Some(true), // true for testing purposes
    };
    /*
    let video = create_video_by_template(&client, payload.clone()).await?;
    */
    let json = json!({
      "template":payload,
      "test_video":"video"
    });

    Ok(Json(json))
}
