use anyhow::Result;
use async_trait::async_trait;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use tokio::fs;
use url::Url;

use super::{super::utils, Extractor};
use crate::media::{image::Image, Media};

pub struct Instagram;

#[async_trait]
impl Extractor for Instagram {
    fn patterns(&self) -> RegexSet {
        RegexSet::new(&[r"(http:|https:)?(//)?(www.)?(instagram.com)/p/(\S+)/?"]).unwrap()
    }

    async fn get_media(&self, url: &str) -> Result<Media> {
        let page = reqwest::get(url).await?.text().await?;

        let re = Regex::new(r#"(\{"config":.+?"\});"#).unwrap();

        let instagram_info = re
            .captures(&page)
            .unwrap()
            .get(1)
            .unwrap()
            .as_str()
            .replace(r"\u0026", "&");

        fs::write("temp/insta.json", &instagram_info).await?;

        let instagram_info: InstagramInfo = serde_json::from_str(&instagram_info).unwrap();

        Ok(Media::Image(Image {
            title: "".to_string(),
            description: "".to_string(),
        }))
    }
}

#[derive(Deserialize)]
struct InstagramInfo {
    country_code: String,
    language_code: String,
    locale: String,
    entry_data: EntryData,
    hostname: String,
    is_whitelisted_crawl_bot: bool,
    connection_quality_rating: String,
    deployment_stage: String,
    platform: String,
    nonce: String,
    mid_pct: f32,
    zero_data: ZeroData,
    cache_schema_version: i32,
    server_checks: ServerChecks,
    knobx: Knobx,
    to_cache: ToChache,
    device_id: String,
    browser_push_pub_key: String,
    encryption: Encryption,
    is_dev: bool,
    // signal_collection_config: Option<>,
    rollout_hash: String,
    bundle_variant: String,
    frontend_env: String,
}

#[derive(Deserialize)]
struct EntryData {}

#[derive(Deserialize)]
struct ZeroData {}

#[derive(Deserialize)]
struct ServerChecks {}

#[derive(Deserialize)]
struct Knobx {}

#[derive(Deserialize)]
struct ToChache {}

#[derive(Deserialize)]
struct Encryption {}
