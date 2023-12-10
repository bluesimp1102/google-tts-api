use std::time::{ Instant, Duration };

use reqwest::Client;
use serde::Deserialize;
use crate::{ types::credentials::GoogleCredentials, jwt::create_jwt };

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: usize, // Expiration time in seconds
}

pub async fn get_access_token(
    credentials: &GoogleCredentials
) -> Result<(String, Instant), Box<dyn std::error::Error + Sync + Send + 'static>> {
    let jwt = create_jwt(credentials)?;

    let client = Client::new();
    let params = [
        ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
        ("assertion", &jwt),
    ];
    let res = client.post(&credentials.token_uri).form(&params).send().await?;

    let token_response: TokenResponse = res.json().await?;

    let expiration = Instant::now() + Duration::from_secs(token_response.expires_in as u64);
    Ok((token_response.access_token, expiration))
}
