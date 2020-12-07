use anyhow::{anyhow, Result};
use async_trait::async_trait;
use initial_data::InitialData;
use regex::{Regex, RegexSet};
use std::collections::HashMap;
use url::Url;
use ytplayer::PlayerResponse;

use super::{super::utils, Extractor};
use crate::media::{
    video::{Stream, Video},
    video_playlist::VideoPlaylist,
    Media,
};

mod initial_data;
mod ytplayer;

pub struct Youtube;

#[async_trait]
impl Extractor for Youtube {
    fn patterns(&self) -> RegexSet {
        // Probably matches youtube urls, needs more testing
        // The only way this could fail is if the regex string is invalid so unwrapping should be safe
        RegexSet::new(&[
            r"(http:|https:)?(//)?(www\.|m\.)?(youtube.com|youtu.be)/(watch)?(\?v=)?(\S+)?",
        ])
        .unwrap()
    }

    async fn get_media(&self, url: &str) -> Result<Media> {
        let url = Url::parse(url)?;
        let pairs: HashMap<String, String> = url.query_pairs().into_owned().collect();

        if let Some(id) = pairs.get("v") {
            let player_response = get_player_response(id).await?;
            download(&player_response).await?;

            let video_details = player_response.video_details;

            let streams: Vec<Stream> = Vec::new();

            let height: i32 = 0;
            let width: i32 = 0;

            let video = Media::Video(Video {
                title: video_details.title,
                description: Some(video_details.short_description),
                height: height.to_owned(),
                width: width.to_owned(),
                streams,
            });

            Ok(video)
        } else if let Some(list) = pairs.get("list") {
            let initial_data = get_initial_data(list).await?;

            let video_playlist = Media::VideoPlaylist(VideoPlaylist {
                title: initial_data.metadata.playlist_metadata_renderer.title,
            });

            Ok(video_playlist)
        } else {
            return Err(anyhow!("Couldn't find neither an id or list"));
        }
    }
}

/// Extract the player response from the given page
async fn get_player_response(id: &str) -> Result<ytplayer::PlayerResponse> {
    let url = format!(
        "https://www.youtube.com/watch?v={}&{}&bpctr={}",
        id,
        "hl=en",
        utils::since_epoch()
    );

    let page = reqwest::get(&url).await?.text().await?;

    // There are multiple ways to get the player response
    // 99% of the time it's avaible directly in the page via the ytplater config or stored in the window object
    // There are other ways to obtain it but unless the youtube api changes these never fail
    let config_re = Regex::new(r"ytplayer\.config\s*=\s*(\{.+?\});")?;
    let player_response_re =
        Regex::new(r#"window\["ytInitialPlayerResponse"\]\s*=\s*(\{.+?\})\s*;"#)?;

    let player_response = if let Some(c) = config_re.captures(&page) {
        // The config is encoded in a strange way, this just removes all the escape sequences and makes it a valide json object
        let config = c
            .get(1)
            .unwrap()
            .as_str()
            .replace("\\\\", "\\")
            .replace("\\\"", "\"")
            .replace("}}}}\"}}", "}}}}}}")
            .replace("\"{\"", "{\"");

        let youtube_info: ytplayer::Config = serde_json::from_str(config.as_str()).unwrap();
        // The only usefull information are contained in the player_response
        youtube_info.args.player_response
    } else if let Some(c) = player_response_re.captures(&page) {
        let player_response: ytplayer::PlayerResponse =
            serde_json::from_str(c.get(1).unwrap().as_str())?;

        player_response
    } else {
        // This will likely never happen
        // Either a wrong url was passed or youtube changed its internal api
        return Err(anyhow!("Unable to retrieve player response"));
    };

    // If everything went correctly this should contain all the information about the video
    Ok(player_response)
}

async fn get_initial_data(list: &str) -> Result<initial_data::InitialData> {
    let url = format!(
        "https://www.youtube.com/playlist?list={}&{}&bpctr={}",
        list,
        "hl=en",
        utils::since_epoch()
    );

    let page = reqwest::get(&url).await?.text().await?;

    let initial_data_re = Regex::new(r"ytInitialData\s*=\s*(\{.+?\})\s*;")?;
    let window_initial_data_re = Regex::new(r#"window\["ytInitialData"\]\s*=\s*(\{.+?\})\s*;"#)?;

    let initial_data = if let Some(c) = initial_data_re.captures(&page) {
        let initial_data: InitialData = serde_json::from_str(c.get(1).unwrap().as_str())?;

        initial_data
    } else if let Some(c) = window_initial_data_re.captures(&page) {
        let initial_data: InitialData = serde_json::from_str(c.get(1).unwrap().as_str())?;

        initial_data
    } else {
        return Err(anyhow!("Unable to retrieve config"));
    };

    // println!("{:#?}", &initial_data.contents());

    Ok(initial_data)
}

pub async fn download(player_response: &PlayerResponse) -> Result<()> {
    // let format = config
    //     .args
    //     .player_response
    //     .streaming_data
    //     .formats
    //     .first()
    //     .unwrap();

    // match format {
    //     Formats::Format { url, .. } => {
    //         println!("{}", url);
    //         let resp = reqwest::get(url).await?.bytes().await?;

    //         let mut file = File::create("temp/video.mp4")?;
    //         file.write_all(&resp)?;
    //     }
    //     Formats::CipheredFormat { .. } => println!("Chipered formats are not yet supported"),
    // }

    // let url = "https://www.youtube.com/playlist?list=PLLLTSoKIAK2yYkEGj5S_vfqXEYb_AoKHi";
    // let url = "https://www.youtube.com/watch?v=v1K4EAXe2oo";

    let formats = player_response.streaming_data.formats.first().unwrap();

    match formats {
        ytplayer::Formats::Format { .. } => {}
        ytplayer::Formats::CipheredFormat {
            itag,
            mime_type,
            bitrate,
            width,
            height,
            last_modified,
            content_length,
            quality,
            fps,
            quality_label,
            projection_type,
            average_bitrate,
            audio_quality,
            approx_duration_ms,
            audio_sample_rate,
            audio_channels,
            signature_cipher,
        } => {
            println!("{}", signature_cipher)
        }
    }

    Ok(())
}
