use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct Thumbnails {
    pub thumbnails: Vec<Thumbnail>,
}

#[derive(Serialize, Deserialize)]
pub struct Thumbnail {
    pub url: String,
    pub width: u32,
    pub height: u32,
}
