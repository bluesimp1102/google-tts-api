use serde::{ Deserialize, Serialize };

/// Represents a request to synthesize speech.
///
/// This struct holds the data necessary to make a request to a speech synthesis service.
/// It includes the text to be synthesized, voice parameters, and audio configuration.
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct SynthesizeRequest {
    /// Required. The Synthesizer requires either plain text or SSML as input.
    pub input: SynthesisInput,
    /// The desired voice of the synthesized audio.
    pub voice: VoiceSelectionParams,
    /// The configuration of the synthesized audio.
    pub audioConfig: AudioConfig,
}

/// Represents the input for speech synthesis.
///
/// This struct can contain either plain text or SSML for synthesis.
///
/// JSON representation
/// ```
/// {
///   // Union field input_source can be only one of the following:
///   "text": string,
///   "ssml": string
///   // End of list of possible types for union field input_source.
/// }
/// ```
#[derive(Debug, Serialize)]
pub struct SynthesisInput {
    // Define the structure of the request body according to Google's API
    // The raw text to be synthesized.
    pub text: Option<String>,
    // The Speech Synthesis Markup Language (SSML) document to be synthesized. The SSML document must be valid and well-formed. Otherwise the RPC will fail and return [google.rpc.Code.INVALID_ARGUMENT](https://cloud.google.com/text-to-speech/docs/reference/rest/Shared.Types/Code#ENUM_VALUES.INVALID_ARGUMENT). For more information, see SSML.
    pub ssml: Option<String>,
}

/// Parameters for voice selection in speech synthesis.
///
/// This struct defines the language, name, gender, and custom voice parameters for synthesis.
///
/// JSON representation
/// ```
/// {
///     "languageCode": string,
///     "name": string,
///     "ssmlGender": enum (SsmlVoiceGender),
///     "customVoice": {
///       object (CustomVoiceParams)
///     }
/// }
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Default)]
pub struct VoiceSelectionParams {
    /// Required. The language (and potentially also the region) of the voice expressed as a BCP-47 language tag, e.g. "en-US". This should not include a script tag (e.g. use "cmn-cn" rather than "cmn-Hant-cn"), because the script will be inferred from the input provided in the SynthesisInput. The TTS service will use this parameter to help choose an appropriate voice. Note that the TTS service may choose a voice with a slightly different language code than the one selected; it may substitute a different region (e.g. using en-US rather than en-CA if there isn't a Canadian voice available), or even a different language, e.g. using "nb" (Norwegian Bokmal) instead of "no" (Norwegian)".
    #[serde(default = "default_language_code")]
    pub languageCode: String,
    /// The name of the voice. If not set, the service will choose a voice based on the other parameters such as languageCode and gender.
    pub name: Option<String>,
    /// The preferred gender of the voice. If not set, the service will choose a voice based on the other parameters such as languageCode and name. Note that this is only a preference, not requirement; if a voice of the appropriate gender is not available, the synthesizer should substitute a voice with a different gender rather than failing the request.
    #[serde(default)]
    pub ssmlGender: SsmlVoiceGender,
    /// The configuration for a custom voice. If CustomVoiceParams.model is set, the service will choose the custom voice matching the specified configuration.
    pub customVoice: Option<CustomVoiceParams>,
}

/// Default language code used in `VoiceSelectionParams`.
pub fn default_language_code() -> String {
    "en-US".to_string()
}

/// Enum representing the gender of the voice in speech synthesis.
#[derive(Serialize, Debug, Default)]
pub enum SsmlVoiceGender {
    /// Gender unspecified.
    #[serde(rename = "SSML_VOICE_GENDER_UNSPECIFIED")]
    SsmlVoiceGenderUnspecified,
    /// Male voice.
    #[serde(rename = "MALE")]
    Male,
    /// Female voice.
    #[serde(rename = "FEMALE")]
    Female,
    /// Neutral gender voice.
    #[default]
    #[serde(rename = "NEUTRAL")]
    Neutral,
}

/// Custom voice parameters for speech synthesis.
///
/// This struct defines the custom voice model and usage for the synthesized speech.
///
/// JSON representation
/// ```
/// {
///   "model": string,
///   "reportedUsage": enum (ReportedUsage)
/// }
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct CustomVoiceParams {
    /// Required. The name of the AutoML model that synthesizes the custom voice.
    pub model: String,
    /// Optional. The usage of the synthesized audio to be reported.
    pub reportedUsage: Option<ReportedUsage>,
}

/// The usage of the synthesized audio. Usage does not affect billing.
#[derive(Debug, Serialize)]
pub enum ReportedUsage {
    /// Request with reported usage unspecified will be rejected.
    ReportedUsageUnspecified,
    /// For scenarios where the synthesized audio is not downloadable and can only be used once. For example, real-time request in IVR system.
    Realtime,
    /// For scenarios where the synthesized audio is downloadable and can be reused. For example, the synthesized audio is downloaded, stored in customer service system and played repeatedly.
    Offline,
}

/// Audio configuration for speech synthesis.
///
/// This struct defines the encoding, rate, pitch, volume, and other audio settings for the synthesized speech.
///
/// JSON Representation
/// ```
/// {
///   "audioEncoding": enum (AudioEncoding),
///   "speakingRate": number,
///   "pitch": number,
///   "volumeGainDb": number,
///   "sampleRateHertz": integer,
///   "effectsProfileId": [
///     string
///   ]
/// }
/// ```
#[allow(non_snake_case)]
#[derive(Debug, Serialize, Default)]
pub struct AudioConfig {
    /// Required. The format of the audio byte stream.
    #[serde(default)]
    pub audioEncoding: AudioEncoding,
    /// Optional. Input only. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal native speed supported by the specific voice. 2.0 is twice as fast, and 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any other values < 0.25 or > 4.0 will return an error.
    pub speakingRate: Option<f32>,
    /// Optional. Input only. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20 semitones from the original pitch. -20 means decrease 20 semitones from the original pitch.
    pub pitch: Option<f32>,
    /// Optional. Input only. Volume gain (in dB) of the normal native volume supported by the specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB) will play at approximately half the amplitude of the normal native signal amplitude. A value of +6.0 (dB) will play at approximately twice the amplitude of the normal native signal amplitude. Strongly recommend not to exceed +10 (dB) as there's usually no effective increase in loudness for any value greater than that.
    pub volumeGainDb: Option<f32>,
    /// Optional. The synthesis sample rate (in hertz) for this audio. When this is specified in SynthesizeSpeechRequest, if this is different from the voice's natural sample rate, then the synthesizer will honor this request by converting to the desired sample rate (which might result in worse audio quality), unless the specified sample rate is not supported for the encoding chosen, in which case it will fail the request and return [google.rpc.Code.INVALID_ARGUMENT](https://cloud.google.com/text-to-speech/docs/reference/rest/Shared.Types/Code#ENUM_VALUES.INVALID_ARGUMENT.)
    pub sampleRateHertz: Option<i32>,
    /// Optional. Input only. An identifier which selects 'audio effects' profiles that are applied on (post synthesized) text to speech. Effects are applied on top of each other in the order they are given. See [audio profiles](https://cloud.google.com/text-to-speech/docs/audio-profiles) for current supported profile ids.
    pub effectsProfileId: Option<Vec<String>>,
}

/// Enum representing the audio encoding for speech synthesis.
#[derive(Debug, Serialize, Default)]
pub enum AudioEncoding {
    /// Not specified. Will return result [google.rpc.Code.INVALID_ARGUMENT](https://cloud.google.com/text-to-speech/docs/reference/rest/Shared.Types/Code#ENUM_VALUES.INVALID_ARGUMENT).
    #[serde(rename = "AUDIO_ENCODING_UNSPECIFIED")]
    AudioEncodingUnspecified,
    #[default]
    /// MP3 audio at 32kbps.
    #[serde(rename = "MP3")]
    Mp3,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM). Audio content returned as LINEAR16 also contains a WAV header.
    /// Linear16,
    /// Opus encoded audio wrapped in an ogg container. The result will be a file which can be played natively on Android, and in browsers (at least Chrome and Firefox). The quality of the encoding is considerably higher than MP3 while using approximately the same bitrate.
    #[serde(rename = "OGGOPUS")]
    OggOpus,
    // 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law. Audio content returned as MULAW also contains a WAV header.
    #[serde(rename = "MULAW")]
    Mulaw,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/A-law. Audio content returned as ALAW also contains a WAV header.
    #[serde(rename = "ALAW")]
    Alaw,
}

/// Represents a response from a speech synthesis request.
#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
pub struct SynthesizeResponse {
    /// Contains the synthesized audio content.
    ///
    /// This field stores the audio data bytes encoded as specified in the request.
    /// For encodings that are wrapped in containers (e.g., MP3, OGG_OPUS), it includes
    /// the appropriate header. For LINEAR16 audio, it contains a WAV header.
    /// In JSON representations, the audio content is base64-encoded.
    pub audioContent: String,
}
