use clap::{App, Arg, SubCommand};
use regex::Regex;
use std::fs;

mod utils;
mod ytplayer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("nyaa")
        .version("1.0")
        .arg(Arg::with_name("id").required(true))
        .get_matches();

    let id = matches.value_of("id").unwrap();

    let config = get_info(id).await?;
    // println!("{}", config.args.player_response.video_details.title);
    fs::write(
        "temp/moe.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .unwrap();

    let streaming_data = config.args.player_response.streaming_data;

    Ok(())
}

async fn get_info(id: &str) -> Result<ytplayer::Config, Box<dyn std::error::Error>> {
    let full = format!(
        "https://www.youtube.com/watch?v={}&{}&bpctr={}",
        id,
        "hl=en",
        utils::since_epoch()
    );

    let page = reqwest::get(&full).await?.text().await?;

    let re = Regex::new(r";ytplayer\.config\s*=\s*(\{.+?\});").unwrap();
    let config = re.captures(&page).unwrap().get(1).unwrap().as_str();

    let config = config
        .replace("\\\\", "\\")
        .replace("\\\"", "\"")
        .replace("}}}}\"}}", "}}}}}}")
        .replace("\"{\"", "{\"");

    fs::write("temp/config.json", &config).unwrap();

    let config: ytplayer::Config = serde_json::from_str(config.as_str()).unwrap();

    Ok(config)
}
