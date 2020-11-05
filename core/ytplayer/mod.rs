use serde::{Deserialize, Serialize};

mod streaming_data;
pub use streaming_data::*;
mod video_details;
pub use video_details::*;
mod captions;
pub use captions::*;

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
    pub streaming_data: StreamingData,
    #[serde(rename = "videoDetails")]
    pub video_details: VideoDetails,
    pub captions: Option<Captions>,
}
