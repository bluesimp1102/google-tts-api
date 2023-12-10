// src/services/synthesize.rs

use std::{ future::{ IntoFuture, Future }, pin::Pin };

use async_trait::async_trait;
use reqwest::Client;

use crate::types::synthesize::{
    AudioConfig,
    VoiceSelectionParams,
    SynthesizeResponse,
    SynthesizeRequest,
    SynthesisInput,
    default_language_code,
    SsmlVoiceGender,
};

pub struct SynthesizeSpeechBuilder {
    access_token: String,
    input_text: String,
    voice: VoiceSelectionParams,
    audio_config: AudioConfig,
}

impl SynthesizeSpeechBuilder {
    pub fn new(access_token: String, input_text: String) -> Self {
        SynthesizeSpeechBuilder {
            access_token,
            input_text,
            voice: VoiceSelectionParams {
                languageCode: default_language_code(),
                ssmlGender: SsmlVoiceGender::Neutral,
                ..Default::default()
            },
            audio_config: AudioConfig::default(),
        }
    }

    pub fn input_text(mut self, text: &str) -> Self {
        self.input_text = text.to_string();
        self
    }

    pub fn voice_params(mut self, voice: VoiceSelectionParams) -> Self {
        self.voice = voice;
        self
    }

    pub fn audio_config(mut self, config: AudioConfig) -> Self {
        self.audio_config = config;
        self
    }

    pub async fn exec(self) -> Result<SynthesizeResponse, SynthesisError> {
        self.into_future().await
    }
}

#[async_trait]
impl IntoFuture for SynthesizeSpeechBuilder {
    type Output = Result<SynthesizeResponse, SynthesisError>;
    type IntoFuture = Pin<Box<dyn Future<Output = Self::Output> + Send>>; // Add + Send here

    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let client = Client::new();
            let request = SynthesizeRequest {
                input: SynthesisInput {
                    text: self.input_text,
                    // ssml: None,
                },
                voice: self.voice,
                audioConfig: AudioConfig::default(),
            };

            // Make the request to the Google TTS API
            let response = client
                .post("https://texttospeech.googleapis.com/v1/text:synthesize")
                .bearer_auth(&self.access_token)
                .json(&request)
                .send().await?;

            // Assuming `response` is a reqwest::Response object
            if response.status() != reqwest::StatusCode::OK {
                let error_body = response.text().await.unwrap_or_default();
                println!("Error response body: {}", error_body);

                return Err(
                    SynthesisError::ApiError(format!("error from google api {}", error_body))
                );
            }

            // println!("request {:?}", request);

            Ok(response.json::<SynthesizeResponse>().await?)
        })
    }
}

#[derive(Debug)]
pub enum SynthesisError {
    RequestError(reqwest::Error),
    ApiError(String), // Assuming API errors are returned as a string
    // You can add more error variants as needed
}

impl From<reqwest::Error> for SynthesisError {
    fn from(error: reqwest::Error) -> Self {
        SynthesisError::RequestError(error)
    }
}

impl std::fmt::Display for SynthesisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynthesisError::RequestError(e) => write!(f, "Request error: {}", e),
            SynthesisError::ApiError(e) => write!(f, "API error: {}", e),
        }
    }
}

impl std::error::Error for SynthesisError {}

unsafe impl Send for SynthesisError {}
