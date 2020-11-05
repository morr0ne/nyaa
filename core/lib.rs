use anyhow::Result;
use regex::Regex;
use std::fs;

mod utils;
pub mod ytplayer;

pub async fn get_info(id: &str) -> Result<ytplayer::Config> {
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

    // fs::write("temp/config.json", &config).unwrap();
    // let config = fs::read_to_string("temp/config.json").unwrap();

    let config: ytplayer::Config = serde_json::from_str(config.as_str()).unwrap();

    Ok(config)
}

pub async fn download(config: &ytplayer::Config) -> Result<()> {
    fs::write(
        "temp/moe.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )
    .unwrap();

    Ok(())
}
