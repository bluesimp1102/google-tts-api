use songbird::input::Input;
use crate::services::composer::GoogleTtsComposer;
impl From<SynthesizeResponse> for Input {
    /// Converts a [SynthesizeResponse] into [songbird::input::Input].
    ///
    /// This implementation is used to create a [songbird::input::Input] instance from
    /// a [SynthesizeResponse]. It utilizes [GoogleTtsComposer] to handle the conversion
    /// of the audio content from [SynthesizeResponse].
    ///
    /// ## Arguments
    ///
    /// * `value` - The [SynthesizeResponse] instance containing the synthesized audio data.
    ///
    /// ## Returns
    ///
    /// Returns an [Input] instance which can be used with the [songbird] library
    /// for voice processing in Discord bots or similar applications.
    ///
    /// ## Example
    ///
    /// ```
    /// let response = SynthesizeResponse { audioContent: "base64-encoded-audio" };
    /// let input = Input::from(response);
    /// // Now `input` can be used with songbird's audio handling functions.
    /// ```
    fn from(value: SynthesizeResponse) -> Self {
        let composer = GoogleTtsComposer::new(value.audioContent.clone());
        Input::Lazy(Box::new(composer))
    }
}
