use serde::{Deserialize, Serialize};

pub mod captions;
pub mod streaming_data;
pub mod video_details;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub args: Args,
}
#[derive(Serialize, Deserialize)]
pub struct Args {
    pub player_response: PlayerResponse,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerResponse {
    #[serde(rename = "streamingData")]
    pub streaming_data: streaming_data::StreamingData,
    #[serde(rename = "videoDetails")]
    pub video_details: video_details::VideoDetails,
    pub captions: Option<captions::Captions>,
}
