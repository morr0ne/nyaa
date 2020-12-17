pub mod extractors;
pub mod media;

mod utils;

use anyhow::{anyhow, Result};
use extractors::{Instagram, Youtube};
use media::Media;
use url::Url;

pub struct DownloadOption {}

impl Default for DownloadOption {
    fn default() -> Self {
        Self {}
    }
}

/// Downloads the given media at the specified path
pub async fn download(media: Media, path: &str, download_option: DownloadOption) -> Result<()> {
    Ok(())
}

/// Extracts the site info into a more generic type that can either directly used or passed to [download]
pub async fn get_media(
    // Ideally the user should be able to load any arbitrary extractor but this isn't a priority
    // extractors: Vec<Box<dyn extractors::Extractor>>,
    url: &str,
) -> Result<Media> {
    // Check if url is valid otherwise returns the url ParseError but we might want to add a custom error
    Url::parse(url)?;
    // The extractor are hardcoded here but we probably want to make them avaible through features
    let extractors: Vec<Box<dyn extractors::Extractor>> = vec![Box::new(Youtube),Box::new(Instagram)];

    // Initialize to None so that if no extractors are found an error can be returned
    let mut media: Option<Media> = None;
    for extractor in &extractors {
        let patterns = extractor.patterns();
        if patterns.is_match(url) {
            media = Some(extractor.get_media(url).await?);
            break;
        }
    }

    match media {
        Some(media) => Ok(media),
        None => Err(anyhow!("Couldn't find an extractor")),
    }
}
