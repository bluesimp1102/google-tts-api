# Google Text To Speech API in Rust

## Overview

`google-tts-api` is a Rust library providing an interface to Google's Text To Speech (TTS) API. This library enables Rust applications to convert text into natural-sounding speech using Google's state-of-the-art deep learning models.

## Features

- Interface to Google's TTS API using Rust.
- Support for various languages and voices.
- Customization of speech parameters (e.g., pitch, speed, volume).
- Support for both plaintext and SSML inputs.

## Installation

To use `google-tts-api` in your project, add it to your `Cargo.toml` file:

```toml
[dependencies]
google-tts-api = {}
```

## Usage

Here's a simple example to demonstrate how to use google-tts-api:

```rust
use google_tts_api::{TtsClient, SynthesizeInput, VoiceSelectionParams, AudioConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let credentials = /* Load your Google Cloud credentials */;
    let tts_client = TtsClient::new(&credentials).await?;

    let input = SynthesizeInput { text: "Hello, world!".to_string() };
    let voice = VoiceSelectionParams::default();
    let audio_config = AudioConfig::default();

    let response = tts_client.synthesize_text(input, voice, audio_config).await?;

    // Process the response
    // ...

    Ok(())
}
```

## Configuration

Before using google-tts-api, you'll need to set up your Google Cloud credentials. Refer to the [Google Cloud authentication](https://cloud.google.com/docs/authentication/getting-started) guide for instructions on obtaining your credentials file.
