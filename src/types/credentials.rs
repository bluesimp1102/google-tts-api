// src/credentials.rs
use serde::Deserialize;
use std::{ fs::File, io::BufReader };

#[derive(Deserialize, Clone)]
pub struct GoogleCredentials {
    pub private_key: String,
    pub client_email: String,
    pub token_uri: String,
}

pub fn read_credentials(
    file: File
) -> Result<GoogleCredentials, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let reader = BufReader::new(file);
    let creds = serde_json::from_reader(reader)?;
    Ok(creds)
}

#[derive(Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    // Include other fields if necessary
}
