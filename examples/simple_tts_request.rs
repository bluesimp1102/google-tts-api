extern crate google_tts_api;

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
