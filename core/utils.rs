use anyhow::Result;
use reqwest::Client;
use std::time::{SystemTime, UNIX_EPOCH};

use tokio::fs::File;
use tokio::prelude::*;

/// Time since "1970-01-01 00:00:00 UTC" (Unix epoch) in seconds
pub fn since_epoch() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

pub async fn download_file() -> Result<()> {
    let client = reqwest::Client::new();

    // let url = "https://i.pximg.net/img-original/img/2017/04/01/02/03/26/62190756_p0.jpg";
    let url = "http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_1080p_60fps_stereo_abl.mp4";

    // let res = client
    //     .get(url)
    //     .header("Referer", "http://www.pixiv.net/")
    //     .send()
    //     .await?
    // .bytes()
    // .await?;

    // let mut file = File::create("temp/image.jpg").await?;
    // file.write_all(&res).await?;

    let mut res = client
        .get(url)
        // .header("Referer", "http://www.pixiv.net/")
        .send()
        .await?;

    while let Some(chunk) = res.chunk().await? {
        println!("Chunk: {:?}", chunk);
        println!("\n---------------------------------------\n");
    }

    Ok(())
}
