use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Format>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormat>,
}

#[derive(Serialize, Deserialize)]
pub struct Format {
    pub itag: u32,
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    pub bitrate: u32,
    pub width: u32,
    pub height: u32,
    #[serde(rename = "lastModified")]
    pub last_modified: String,
    #[serde(rename = "contentLength")]
    pub content_length: Option<String>,
    pub quality: String,
    pub fps: u32,
    #[serde(rename = "qualityLabel")]
    pub quality_label: String,
    #[serde(rename = "projectionType")]
    pub projection_type: String,
    #[serde(rename = "averageBitrate")]
    pub average_bitrate: Option<u32>,
    #[serde(rename = "audioQuality")]
    pub audio_quality: String,
    #[serde(rename = "approxDurationMs")]
    pub approx_duration_ms: String,
    #[serde(rename = "audioSampleRate")]
    pub audio_sample_rate: String,
    #[serde(rename = "audioChannels")]
    pub audio_channels: u32,
}
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum AdaptiveFormat {
    Video {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
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
    Audio {
        itag: u32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: String,
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
        audio_quality: String,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: u32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
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
