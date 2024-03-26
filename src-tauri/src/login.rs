use reqwest;
use serde::{Deserialize, Serialize};
use colored::*;
use berry_lib::file_sys::app_bin;

#[derive(Serialize)]
struct DeviceAuthRequest {
    client_id: String,
    scope: String,
}



pub enum DeviceAuthStatus {
    Authorized(app_bin::DeviceAuthBinary),
    Pending(String),
    InvalidCode(String),
    InvalidRefreshToken(String),
}

#[tauri::command]
pub async fn request_device_authorization() -> Result<app_bin::DeviceAuthBinary, String> {
    println!("{}", "Requesting Device Authorization".green());

    let client = reqwest::Client::new();
    let scopes = [
        "analytics:read:extensions",
        "analytics:read:games",
        "bits:read",
        "channel:edit:commercial",
        "channel:manage:broadcast",
        "channel:manage:extensions",
        "channel:manage:moderators",
        "channel:manage:polls",
        "channel:manage:predictions",
        "channel:manage:raids",
        "channel:manage:redemptions",
        "channel:manage:schedule",
        "channel:manage:videos",
        "channel:read:editors",
        "channel:read:goals",
        "channel:read:hype_train",
        "channel:read:polls",
        "channel:read:predictions",
        "channel:read:redemptions",
        "channel:read:stream_key",
        "channel:read:subscriptions",
        "channel:read:vips",
        "channel:manage:vips",
        "clips:edit",
        "moderation:read",
        "moderator:manage:announcements",
        "moderator:manage:automod",
        "moderator:read:automod_settings",
        "moderator:manage:automod_settings",
        "moderator:manage:banned_users",
        "moderator:read:blocked_terms",
        "moderator:manage:blocked_terms",
        "moderator:manage:chat_messages",
        "moderator:read:chat_settings",
        "moderator:manage:chat_settings",
        "user:edit",
        "user:edit:follows",
        "user:manage:blocked_users",
        "user:read:blocked_users",
        "user:read:broadcast",
        "user:manage:chat_color",
        "user:read:email",
        "user:read:follows",
        "user:read:subscriptions",
        "user:manage:whispers",
        "channel:moderate",
        "chat:read",
        "chat:edit",
        "whispers:read",
        "whispers:edit",
        "moderator:read:followers",
    ]
    .join(" ");

    let request_body = DeviceAuthRequest {
        client_id: "mi58wuxiqzwi4x697zqs7843lq3xh8".to_string(),
        scope: scopes,
    };

    let response = client
        .post("https://id.twitch.tv/oauth2/device")
        .form(&request_body)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    match response.status().as_u16() {
        200 => {
            let response_body: app_bin::DeviceAuthBinary = response.json().await.map_err(|e| e.to_string())?;

            let new_device_auth = app_bin::DeviceAuthBinary::new(
                response_body.device_code.clone(),
                response_body.expires_in.clone(),
                response_body.interval.clone(),
                response_body.user_code.clone(),
                response_body.verification_uri.clone(),
                true,
            );

            Ok(response_body)
        }
        400 => {
            let response_body: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
            let error_message = response_body["error_description"].as_str().unwrap_or("Unknown Error");



            Err(error_message.to_string())
        }
        _ => {
            let error_message = format!("Unexpected response status: {}", response.status());
            Err(error_message)
        }
    }
}

