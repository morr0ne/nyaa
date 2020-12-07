// This file contains all the deserialization information for ytplayer.config
// Having a 300+ loc file is less than ideal
// until I figure out a proper directory structure it will remain this way

use serde::Deserialize;

/// Extracted from yplayer.config, no really useful on its own as all the information are contained in the PlayerResponse
#[derive(Deserialize)]
pub struct Config {
    pub args: Args,
}
/// Extracted from yplayer.config.args, no really useful on its own as all the information are contained in the PlayerResponse
#[derive(Deserialize)]
pub struct Args {
    pub player_response: PlayerResponse,
}

/// Contains all the information about the video
#[derive(Deserialize)]
pub struct PlayerResponse {
    #[serde(rename = "streamingData")]
    pub streaming_data: StreamingData,
    #[serde(rename = "videoDetails")]
    pub video_details: VideoDetails,
    pub captions: Option<Captions>,
}

#[derive(Deserialize)]
pub struct StreamingData {
    #[serde(rename = "expiresInSeconds")]
    pub expires_in_seconds: String,
    pub formats: Vec<Formats>,
    #[serde(rename = "adaptiveFormats")]
    pub adaptive_formats: Vec<AdaptiveFormats>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Formats {
    Format {
        itag: i32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
        width: i32,
        height: i32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: i32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<i32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: i32,
    },
    CipheredFormat {
        itag: i32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
        width: i32,
        height: i32,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: Option<String>,
        quality: String,
        fps: i32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: Option<i32>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: i32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}
#[derive(Deserialize)]
#[serde(untagged)]
pub enum AdaptiveFormats {
    Video {
        itag: i32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
        width: i32,
        height: i32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: i32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: i32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
    },
    CipheredVideo {
        itag: i32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
        width: i32,
        height: i32,
        #[serde(rename = "initRange")]
        init_range: InitRange,
        #[serde(rename = "indexRange")]
        index_range: InitRange,
        #[serde(rename = "lastModified")]
        last_modified: String,
        #[serde(rename = "contentLength")]
        content_length: String,
        quality: String,
        fps: i32,
        #[serde(rename = "qualityLabel")]
        quality_label: String,
        #[serde(rename = "projectionType")]
        projection_type: String,
        #[serde(rename = "averageBitrate")]
        average_bitrate: i32,
        #[serde(rename = "colorInfo")]
        color_info: Option<ColorInfo>,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
    Audio {
        itag: i32,
        url: String,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
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
        average_bitrate: i32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: i32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
    },

    CipheredAudio {
        itag: i32,
        #[serde(rename = "mimeType")]
        mime_type: mime::MediaType,
        bitrate: i32,
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
        average_bitrate: i32,
        #[serde(rename = "highReplication")]
        high_replication: Option<bool>,
        #[serde(rename = "audioQuality")]
        audio_quality: AudioQuality,
        #[serde(rename = "approxDurationMs")]
        approx_duration_ms: String,
        #[serde(rename = "audioSampleRate")]
        audio_sample_rate: String,
        #[serde(rename = "audioChannels")]
        audio_channels: i32,
        #[serde(rename = "loudnessDb")]
        loudness_db: f32,
        #[serde(rename = "signatureCipher")]
        signature_cipher: String,
    },
}

#[derive(Deserialize)]
pub struct InitRange {
    pub start: String,
    pub end: String,
}

#[derive(Deserialize)]
pub struct ColorInfo {
    pub primaries: String,
    #[serde(rename = "transferCharacteristics")]
    pub transfer_characteristics: String,
    #[serde(rename = "matrixCoefficients")]
    pub matrix_coefficients: String,
}

#[derive(Deserialize)]
pub enum AudioQuality {
    #[serde(rename = "AUDIO_QUALITY_LOW")]
    AudioQualityLow,
    #[serde(rename = "AUDIO_QUALITY_MEDIUM")]
    AudioQualityMedium,
}

#[derive(Deserialize)]
pub struct VideoDetails {
    #[serde(rename = "videoId")]
    pub video_id: String,
    pub title: String,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: String,
    pub keywords: Option<Vec<String>>,
    #[serde(rename = "channelId")]
    pub channel_id: String,
    #[serde(rename = "isOwnerViewing")]
    pub is_owner_viewing: bool,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    #[serde(rename = "isCrawlable")]
    pub is_crawlable: bool,
    pub thumbnail: Thumbnails,
    #[serde(rename = "averageRating")]
    pub average_rating: f32,
    #[serde(rename = "allowRatings")]
    pub allow_ratings: bool,
    #[serde(rename = "viewCount")]
    pub view_count: String,
    pub author: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    #[serde(rename = "isUnpluggedCorpus")]
    pub is_unplugged_corpus: bool,
    #[serde(rename = "isLiveContent")]
    pub is_live_content: bool,
}

#[derive(Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: i32,
    pub height: i32,
}

#[derive(Deserialize)]
pub struct Captions {
    #[serde(rename = "playerCaptionsRenderer")]
    pub player_captions_renderer: PlayerCaptionsRenderer,
    #[serde(rename = "playerCaptionsTracklistRenderer")]
    pub player_captions_tracklist_renderer: PlayerCaptionsTracklistRenderer,
}

#[derive(Deserialize)]
pub struct PlayerCaptionsRenderer {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub visibility: String,
}

#[derive(Deserialize)]
pub struct PlayerCaptionsTracklistRenderer {
    #[serde(rename = "captionTracks")]
    pub caption_tracks: Vec<CaptionTrack>,
}

#[derive(Deserialize)]
pub struct CaptionTrack {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub name: Name,
    #[serde(rename = "vssId")]
    pub vss_id: String,
    #[serde(rename = "languageCode")]
    pub language_code: String,
    pub kind: Option<String>,
    #[serde(rename = "isTranslatable")]
    pub is_translatable: bool,
}

#[derive(Deserialize)]
pub struct Name {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}
