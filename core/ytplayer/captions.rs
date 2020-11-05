use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Captions {
    #[serde(rename = "playerCaptionsRenderer")]
    pub player_captions_renderer: PlayerCaptionsRenderer,
    #[serde(rename = "playerCaptionsTracklistRenderer")]
    pub player_captions_tracklist_renderer: PlayerCaptionsTracklistRenderer,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerCaptionsRenderer {
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    pub visibility: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerCaptionsTracklistRenderer {
    #[serde(rename = "captionTracks")]
    pub caption_tracks: Vec<CaptionTrack>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Name {
    #[serde(rename = "simpleText")]
    pub simple_text: String,
}
