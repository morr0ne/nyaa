use anyhow::Result;
use async_trait::async_trait;
use regex::{Regex, RegexSet};
use serde::Deserialize;
use std::collections::HashMap;
// use std::fs;

use super::Extractor;
use crate::media::Media;

pub struct Pixiv;

#[async_trait]
impl Extractor for Pixiv {
    fn patterns(&self) -> RegexSet {
        RegexSet::new(&[r"nothing"]).unwrap()
    }

    async fn get_media(&self, url: &str) -> Result<Media> {
        let _pixiv_info = get_pixiv_info(url).await?;
        todo!()
    }
}

pub async fn get_pixiv_info(url: &str) -> Result<PixivInfo> {
    let client = reqwest::Client::new();
    // let url = "https://www.pixiv.net/en/artworks/62190756";

    let page = client.get(url).send().await?.text().await?;

    let re = Regex::new(r#"<meta.*id="meta-global-data".*content='(\{.+?)'"#).unwrap();

    let config = re.captures(&page).unwrap().get(1).unwrap().as_str();

    let pixiv_info: PixivInfo = serde_json::from_str(config)?;

    Ok(pixiv_info)
}

pub async fn _download() -> Result<()> {
    // let url = "https://i.pximg.net/img-original/img/2017/04/01/02/03/26/62190756_p0.jpg";

    // let resp = client
    //     .get(url)
    //     .header("Referer", "http://www.pixiv.net/")
    //     .send()
    //     .await?
    //     .bytes()
    //     .await?;

    // let mut file = File::create("temp/image.jpg")?;
    // file.write_all(&resp)?;

    Ok(())
}

#[derive(Deserialize)]
pub struct PixivInfo {
    pub timestamp: String,
    pub illust: HashMap<String, Illust>,
    pub user: HashMap<String, User>,
}

#[derive(Deserialize)]
pub struct Illust {
    #[serde(rename = "illustId")]
    pub illust_id: String,
    #[serde(rename = "illustTitle")]
    pub illust_title: String,
    #[serde(rename = "illustComment")]
    pub illust_comment: String,
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(rename = "illustType")]
    pub illust_type: i32,
    #[serde(rename = "createDate")]
    pub create_date: String,
    #[serde(rename = "uploadDate")]
    pub upload_date: String,
    pub restrict: i32,
    #[serde(rename = "xRestrict")]
    pub x_restrict: i32,
    pub sl: i32,
    pub urls: Urls,
    pub tags: Tags,
    pub alt: String,
    #[serde(rename = "storableTags")]
    pub storable_tags: Vec<String>,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "userName")]
    pub user_name: String,
    #[serde(rename = "userAccount")]
    pub user_account: String,
    #[serde(rename = "userIllusts")]
    pub user_illusts: UserIllusts,
    #[serde(rename = "likeData")]
    pub like_data: bool,
    pub width: i32,
    pub height: i32,
    #[serde(rename = "pageCount")]
    pub page_count: i32,
    #[serde(rename = "bookmarkCount")]
    pub bookmark_count: i32,
    #[serde(rename = "likeCount")]
    pub like_count: i32,
    #[serde(rename = "commentCount")]
    pub comment_count: i32,
    #[serde(rename = "responseCount")]
    pub response_count: i32,
    #[serde(rename = "viewCount")]
    pub view_count: i32,
    #[serde(rename = "isHowto")]
    pub is_howto: bool,
    #[serde(rename = "isOriginal")]
    pub is_original: bool,
    // imageResponseOutData: [],
    // imageResponseData: [],
    // imageResponseCount: 0,
    // pollData: null,
    // seriesNavData: null,
    // descriptionBoothId: null,
    // descriptionYoutubeId: null,
    // comicPromotion: null,
    // fanboxPromotion: null,
    // contestBanners: [],
    #[serde(rename = "isBookmarkable")]
    pub is_bookmarkable: bool,
    // bookmarkData: null,
    // contestData: null,
    #[serde(rename = "zoneConfig")]
    pub zone_config: ZoneConfig,
    #[serde(rename = "extraData")]
    pub extra_data: ExtraData,
    #[serde(rename = "titleCaptionTranslation")]
    pub title_caption_translation: TitleCaptionTranslation,
    #[serde(rename = "isUnlisted")]
    pub is_unlisted: bool,
    // request: null
    #[serde(rename = "noLoginData")]
    pub no_login_data: NoLoginData,
}

#[derive(Deserialize)]
pub struct User {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub name: String,
    pub image: String,
    #[serde(rename = "imageBig")]
    pub image_big: String,
    pub premium: bool,
    #[serde(rename = "isFollowed")]
    pub is_followed: bool,
    #[serde(rename = "isMypixiv")]
    pub is_mypixiv: bool,
    #[serde(rename = "isBlocking")]
    pub is_blocking: bool,
    pub background: Background,
    // sketchLiveId: null,
    pub partial: i32,
    #[serde(rename = "acceptRequest")]
    pub accept_request: bool,
    // sketchLives: []
}

#[derive(Deserialize)]
pub struct Background {
    // repeat: null,
    // color: null,
    pub url: String,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
}

#[derive(Deserialize)]
pub struct Urls {
    pub mini: String,
    pub thumb: String,
    pub small: String,
    pub regular: String,
    pub original: String,
}

#[derive(Deserialize)]
pub struct Tags {
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    pub writable: bool,
}

#[derive(Deserialize)]
pub struct UserIllusts {}

#[derive(Deserialize)]
pub struct ZoneConfig {}

#[derive(Deserialize)]
pub struct ExtraData {}

#[derive(Deserialize)]
pub struct TitleCaptionTranslation {}

#[derive(Deserialize)]
pub struct NoLoginData {}
