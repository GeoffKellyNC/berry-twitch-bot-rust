//! This module contains the functions for handling the Twitch login process.
//! 
//! It provides the `request_device_authorization` function that sends a request to the Twitch API to request device authorization.

use reqwest;
use serde::{Deserialize, Serialize};
use colored::*;
use berry_lib::{file_sys::app_bin, twitch::twitch_access_token};
use crate::config;



/// The name of the device authentication file.
const DEVICE_AUTH_FILE_NAME: &str = "devauth";


// The request body for the device authorization request.
#[derive(Serialize)]
struct DeviceAuthRequest {
    client_id: String,
    scope: String,
}



/// The status of the device authorization. This is used for authorization token after the device has been authorized.
pub enum DeviceAuthStatus {
    /// The device is authorized.
    Authorized(app_bin::DeviceAuthBinary),
    /// The device is pending authorization.
    Pending(String),
    /// The device authorization has expired.
    InvalidCode(String),
    /// The device authorization has been denied.
    InvalidRefreshToken(String),
}







/// Sends a request to the Twitch API to request device authorization.
/// 
/// # Returns
/// 
/// Returns a `DeviceAuthBinary` struct containing the device code, expiration time, interval, user code, and verification URI.
/// 
/// # Errors
/// 
/// Returns an error message if the request fails.
/// 
/// # Example
/// 
/// ```
/// let device_auth = request_device_authorization().await;
/// ```
///     
#[tauri::command]
pub async fn request_device_authorization() -> Result<app_bin::DeviceAuthBinary, String> {
    println!("{}", "Requesting Device Authorization".green());

    let client = reqwest::Client::new();

    let request_body = DeviceAuthRequest {
        client_id: config::CLIENT_ID.to_string(),
        scope: config::SCOPES.to_string(),
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

            app_bin::write_to_file(&new_device_auth, DEVICE_AUTH_FILE_NAME , app_bin::FileCategory::Config)
            .map_err(|e| e.to_string())?;


            Ok(new_device_auth)
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

