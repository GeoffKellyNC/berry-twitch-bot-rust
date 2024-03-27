use reqwest::Client;
use serde::Deserialize;
use serde_json;

pub enum TwitchTokenError {
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    ApiError(String),
}

impl From<reqwest::Error> for TwitchTokenError {
    fn from(err: reqwest::Error) -> TwitchTokenError {
        TwitchTokenError::RequestError(err)
    }
}

impl From<serde_json::Error> for TwitchTokenError {
    fn from(err: serde_json::Error) -> TwitchTokenError {
        TwitchTokenError::JsonError(err)
    }
}

#[derive(Deserialize, Debug)]
pub struct TwitchAccessToken {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
    pub token_type: String,
}

pub async fn get_twitch_access_token(
    client_id: &str,
    device_code: &str,
    scopes: &str,
    client: &Client,
) -> Result<TwitchAccessToken, TwitchTokenError> {

    let url = "https://id.twitch.tv/oauth2/token";

    let params:[(&str, &str); 4] = [

        ("client_id", client_id),
        ("device_code", device_code),
        ("grant_type", "urn:ietf:params:oauth:grant-type:device_code"),
        ("scope", scopes),
    ];

    let response = client.post(url).form(&params).send().await?;

    if response.status().is_success() {

        let token_data: TwitchAccessToken = response.json().await?;

        Ok(token_data)

    } else {

        let error_message = response.text().await?;

        Err(TwitchTokenError::ApiError(error_message))

    }
}