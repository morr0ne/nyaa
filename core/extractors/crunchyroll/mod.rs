use anyhow::Result;
use std::fs;

pub struct crunchyroll;

pub async fn dcru() -> Result<()> {
    let url = "https://www.crunchyroll.com/rwby/episode-5-sparks-790528";

    let page = reqwest::get(url).await?.text().await?;

    fs::write("temp/page.html", &page)?;

    Ok(())
}
