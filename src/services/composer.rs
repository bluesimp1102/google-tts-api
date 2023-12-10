use async_trait::async_trait;
use songbird::input::{ AudioStreamError, Compose, AudioStream };
use symphonia::core::io::MediaSource;

use super::media_source::GoogleTtsMediaSource;

pub struct GoogleTtsComposer {
    text: String, // Add the access token
}

impl GoogleTtsComposer {
    pub fn new(text: String) -> Self {
        GoogleTtsComposer { text }
    }
}

#[async_trait]
impl Compose for GoogleTtsComposer {
    fn create(&mut self) -> Result<AudioStream<Box<dyn MediaSource>>, AudioStreamError> {
        // Since we are focusing on async, we can simply return an error here
        Err(AudioStreamError::Unsupported)
    }

    async fn create_async(
        &mut self
    ) -> Result<AudioStream<Box<dyn MediaSource>>, AudioStreamError> {
        let source = GoogleTtsMediaSource::new(self.text.clone()).expect(
            "Failed to create media source"
        );
        Ok(AudioStream {
            input: Box::new(source),
            hint: None,
        })
    }

    fn should_create_async(&self) -> bool {
        true
    }
}
