use anyhow::Result;
use clap::{App, Arg};

use nyaa_core::media::Media;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = App::new("nyaa")
        .version("1.0")
        .arg(Arg::with_name("url").required(true))
        .get_matches();

    let url = matches.value_of("url").unwrap();

    println!("{}", url);

    // let url = "https://www.youtube.com/watch?v=v1K4EAXe2oo";
    // let url = "https://www.youtube.com/playlist?list=PLUh9mFlqfTo0wl-jzv9APPg-NPBPD51_r";
    let media = nyaa_core::get_media(url).await?;

    match media {
        Media::Video(ref video) => println!("title: {}", video.title),
        Media::Audio(_) => println!("Found audio"),
        Media::Image(_) => println!("Found image"),
        Media::VideoPlaylist(ref playlist) => println!("title: {}", playlist.title),
        Media::AudioPlaylist(_) => println!("Found audio playlist"),
        Media::ImageAlbum(_) => println!("Found image album"),
        Media::MediaPlaylist(_) => println!("Found media playlist"),
    }

    nyaa_core::download(media, "temp/download", Default::default()).await?;

    Ok(())
}
