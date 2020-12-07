use anyhow::Result;
use regex::Regex;
use std::fs;

pub async fn download() -> Result<()> {
    // let url = "https://soundcloud.com/user-317818400/little-nii-never-gonna-give";
    let url = "https://soundcloud.com/";

    let page = reqwest::get(url).await?.text().await?;

    fs::write("temp/page.cloud.html", &page)?;

    let re = Regex::new(r#"<script[^>]+src="([^"]+)""#)?;
    let client_regex = Regex::new(r#"client_id\s*:\s*"([0-9a-zA-Z]{32})""#).unwrap();

    for src in re.captures_iter(&page) {
        let jssrc = src.get(1).unwrap().as_str();

        println!("{}", jssrc);

        let js = reqwest::get(jssrc).await?.text().await?;

        if let Some(j) = client_regex.captures(&js) {
            let client_id = j.get(0).unwrap().as_str();
            println!("{}", client_id);
        }
    }

    Ok(())
}
