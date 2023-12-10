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
google-tts-api = { "https://github.com/bluesimp1102/google-tts-api" }
```

## Usage

Here's a simple example to demonstrate how to use google-tts-api:

```rust
use std::fs::File;
use google_tts_api::{ client::TextToSpeechClient, types::credentials::read_credentials };

#[tokio::main]
async fn main() {
    let file = File::open("ggl_apis_credentials.json").expect("failed to open credentials file");

    let credentials = read_credentials(file).expect("failed to read credentials from the file");

    let mut tts = TextToSpeechClient::new(&credentials).await.expect(
        "failed to initialize tts client"
    );

    let synthesized_text = tts.synthesize_text("Hello from Jack".to_string()).await.exec().await;

    println!("result: {:?}", synthesized_text);
}
```

## Configuration

Before using google-tts-api, you'll need to set up your Google Cloud credentials. Refer to the [Google Cloud authentication](https://cloud.google.com/docs/authentication/getting-started) guide for instructions on obtaining your credentials file.
