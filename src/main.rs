#![recursion_limit = "256"]

use actix_web::{ http, middleware, web, App, HttpResponse, HttpServer, Result };
use futures::StreamExt;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, Deserialize)]
struct Message_to_skills {
    messageId: u64,
    sessionId: String,
    uuid: Message_UUID,
    payload: Message_to_skills_payload,
}

#[derive(Serialize, Deserialize)]
struct Message_UUID {
    userId: String,
    sub: String,
    userChannel: String,
}

#[derive(Serialize, Deserialize)]
struct Message_to_skills_payload {
    device: Device,
    message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
  original_text: String,
}

#[derive(Serialize, Deserialize)]
struct Device {
    platformType: String,
    platformVersion: String,
    surface: String,
    surfaceVersion: String,
    features: Features,
    capabilities: Capabilities,
    deviceId: String,
    deviceManufacturer: String,
    deviceModel: String,
}

#[derive(Serialize, Deserialize)]
struct Capabilities {
    screen: Capabilities_screen,
}

#[derive(Serialize, Deserialize)]
struct Capabilities_screen {
    available: bool,
    height: i32,
    scale_factor: f32,
    width: i64,
}

#[derive(Serialize, Deserialize)]
struct Features {
    appTypes: Vec<String>,
}

fn get_server_action() -> Value {
    // Send action to app
    json!({
        "type": "INIT",
        "payload": {}
    })
}

async fn app_connector(mut payload: web::Payload) -> Result<HttpResponse> {
    let mut body = web::BytesMut::new();

    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        body.extend_from_slice(&chunk);
    }

    let message_to_skills = serde_json::from_slice::<Message_to_skills>(&body)?;

    let res_json = serde_json::json!({
         "messageName": "ANSWER_TO_USER",
         "messageId": message_to_skills.messageId,
         "sessionId": message_to_skills.sessionId,
         "uuid": message_to_skills.uuid,
         "payload": {
            "device": message_to_skills.payload.device,
            "items": [
                {
                  "command": {
                    "type": "smart_app_data",
                    "action": get_server_action()
                  }
                }
              ],
         },
    });

    Ok(HttpResponse::Ok()
        .set_header(http::header::CONTENT_TYPE, "application/json")
        .body(res_json.to_string()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::new(http::ContentEncoding::Br))
            .route("/app-connector", web::post().to(app_connector))
    })
    .keep_alive(75)
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

