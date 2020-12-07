use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct InitialData {
    pub contents: Contents,
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug)]
pub struct Contents {
    #[serde(rename = "twoColumnBrowseResultsRenderer")]
    pub two_column_browse_results_renderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Deserialize, Debug)]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tab>,
}

#[derive(Deserialize, Debug)]
pub struct Tab {
    #[serde(rename = "tabRenderer")]
    pub tab_renderer: TabRenderer,
}

#[derive(Deserialize, Debug)]
pub struct TabRenderer {
    pub selected: bool,
    pub content: Content,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "sectionListRenderer")]
    pub section_list_renderer: SectionListRenderer,
}

#[derive(Deserialize, Debug)]
pub struct SectionListRenderer {
    pub contents: Vec<SectionListContent>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Deserialize, Debug)]
pub struct SectionListContent {
    #[serde(rename = "itemSectionRenderer")]
    pub item_section_renderer: ItemSectionRenderer,
}

#[derive(Deserialize, Debug)]
pub struct ItemSectionRenderer {
    pub contents: Vec<ItemSectionContent>,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
}

#[derive(Deserialize, Debug)]
pub struct ItemSectionContent {
    #[serde(rename = "playlistVideoListRenderer")]
    pub playlist_video_list_renderer: PlaylistVideoListRenderer,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistVideoListRenderer {
    pub contents: Vec<PlaylistVideoListContent>,
    #[serde(rename = "playlistId")]
    pub playlist_id: String,
    #[serde(rename = "isEditable")]
    pub is_editable: bool,
    #[serde(rename = "canReorder")]
    pub can_reorder: bool,
    #[serde(rename = "trackingParams")]
    pub tracking_params: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistVideoListContent {
    #[serde(rename = "playlistVideoRenderer")]
    pub playlist_video_renderer: PlaylistVideoRenderer,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistVideoRenderer {
    #[serde(rename = "videoId")]
    pub video_id: String,
}

#[derive(Deserialize, Debug)]
pub struct Metadata {
    #[serde(rename = "playlistMetadataRenderer")]
    pub playlist_metadata_renderer: PlaylistMetadataRenderer,
}

#[derive(Deserialize, Debug)]
pub struct PlaylistMetadataRenderer {
    pub title: String,
    #[serde(rename = "androidAppindexingLink")]
    pub android_appindexing_link: String,
    #[serde(rename = "iosAppindexingLink")]
    pub ios_appindexing_link: String,
}

impl InitialData {
    /// Helper fuction to avoid the youtube api nested hell
    pub fn contents(&self) -> &Vec<PlaylistVideoListContent> {
        &self.contents.two_column_browse_results_renderer.tabs[0]
            .tab_renderer
            .content
            .section_list_renderer
            .contents[0]
            .item_section_renderer
            .contents[0]
            .playlist_video_list_renderer
            .contents
    }
}
