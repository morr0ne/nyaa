// mod crunchyroll;
// mod instagram;
// mod pixiv;
// mod soundcloud;
mod youtube;

// pub use instagram::Instagram;
// pub use pixiv::Pixiv;
pub use youtube::Youtube;

use crate::media::Media;

#[async_trait::async_trait]
pub trait Extractor {
    fn patterns(&self) -> regex::RegexSet;
    async fn get_media(&self, url: &str) -> anyhow::Result<Media>;
}
