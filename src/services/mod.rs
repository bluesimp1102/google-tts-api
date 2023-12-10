#[cfg(feature = "enable_songbird")]
pub mod media_source;
#[cfg(feature = "enable_songbird")]
pub mod composer;
#[cfg(feature = "enable_songbird")]
pub mod songbird;

pub mod synthesize;

// pub async fn synthesize_speech(
//     client: &Client, // Use the configured client
//     text: &str,
//     language: &str
// ) -> Result<Vec<u8>, reqwest::Error> {
//     let request_body = SynthesizeRequest {
//         input: todo!(),
//         voice: todo!(),
//         audioConfig: todo!(),
//     };

//     let response = client
//         .post("https://texttospeech.googleapis.com/v1/text:synthesize")
//         .json(&request_body)
//         .send().await?
//         .json::<SynthesizeResponse>().await?;

//     // Process the response to get the audio data
//     todo!()
// }
