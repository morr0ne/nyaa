use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Formats>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormats>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Formats {
    Format {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
    },
    CipheredFormat {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<u32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveFormats {
    Video {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
    },
    CipheredVideo {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        width: u32,
        height: u32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: u32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
    Audio {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
    },

    CipheredAudio {
        itag: u32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: u32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: u32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Serialize, Deserialize)]
pub struct InitRange {
    pub start: String,
    pub end: String,
}

#[derive(Serialize, Deserialize)]
pub struct ColorInfo {
    pub primaries: String,
    #[serde(rename = "transferCharacteristics")]
    pub transfer_characteristics: String,
    #[serde(rename = "matrixCoefficients")]
    pub matrix_coefficients: String,
}

#[derive(Serialize, Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    AudioQualityLow,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    AudioQualityMedium,
}
