use crate::{
    types::credentials::{ GoogleCredentials, read_credentials },
    auth::get_access_token,
    services::synthesize::SynthesizeSpeechBuilder,
};

use std::{ time::Instant, fs::File };

/// Represents an authentication token.
///
/// This struct holds the access token and its expiration time.
pub struct AuthToken {
    access_token: String,
    expiration: Instant,
}

/// Client for Text-to-Speech (TTS) service.
///
/// This struct represents a client that can interact with a TTS service.
/// It holds Google credentials and an authentication token.
pub struct TextToSpeechClient {
    credentials: GoogleCredentials,
    auth_token: AuthToken,
}

impl TextToSpeechClient {
    /// Creates a new Text To Speech client using the provided Google credentials.
    ///
    /// This method initializes a new TTS client by obtaining an access token using
    /// the given credentials.
    ///
    /// # Arguments
    ///
    /// * `credentials` - A reference to [GoogleCredentials] for authentication.
    ///
    /// # Returns
    ///
    /// Returns a [Result] which is either a new [TextToSpeechClient] instance or an error.
    pub async fn new(
        credentials: &GoogleCredentials
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send + 'static>> {
        let (access_token, expiration) = get_access_token(credentials).await?;

        println!("access_token {access_token} expiration {expiration:?}");

        Ok(TextToSpeechClient {
            credentials: credentials.clone(),
            auth_token: AuthToken { access_token, expiration },
        })
    }

    /// Creates a new Text To Speech client using credentials from a configuration file.
    ///
    /// This method reads Google credentials from a file and initializes a new TTS client.
    ///
    /// # Arguments
    ///
    /// * `file` - A [File] instance containing the Google credentials.
    ///
    /// # Returns
    ///
    /// Returns a [Result] which is either a new [TextToSpeechClient] instance or an error.
    pub async fn from_config_file(
        file: File
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send + 'static>> {
        let credentials = read_credentials(file)?;
        let (access_token, expiration) = get_access_token(&credentials).await?;

        println!("access_token {access_token} expiration {expiration:?}");

        Ok(TextToSpeechClient {
            credentials,
            auth_token: AuthToken { access_token, expiration },
        })
    }

    /// Checks and refreshes the authentication token if necessary.
    ///
    /// This method checks if the current token has expired and, if so,
    /// obtains a new one.
    ///
    /// # Returns
    ///
    /// Returns a [Result] which is either () on success or an error.
    pub async fn check_token(
        &mut self
    ) -> Result<(), Box<dyn std::error::Error + Sync + Send + 'static>> {
        if Instant::now() >= self.auth_token.expiration {
            let (new_token, new_expiration) = get_access_token(&self.credentials).await?;
            self.auth_token.access_token = new_token;
            self.auth_token.expiration = new_expiration;
        }
        Ok(())
    }

    /// Retrieves the current access token.
    ///
    /// This method returns the current access token held by the client.
    ///
    /// # Returns
    ///
    /// Returns a [String] containing the access token.
    pub fn get_token(&mut self) -> String {
        self.auth_token.access_token.clone()
    }

    /// Synthesizes text into speech.
    ///
    /// This method takes input text and returns a builder for synthesizing speech
    /// using the TextToSpeech service.
    ///
    /// # Arguments
    ///
    /// * `input_text` - The text to be synthesized into speech.
    ///
    /// # Returns
    ///
    /// Returns a [SynthesizeSpeechBuilder] instance for further configuration and synthesis.
    pub async fn synthesize_text(&mut self, input_text: String) -> SynthesizeSpeechBuilder {
        let _ = self.check_token().await;
        SynthesizeSpeechBuilder::new(self.auth_token.access_token.clone(), input_text)
    }
}
