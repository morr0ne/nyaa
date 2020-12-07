pub struct Video {
    pub title: String,
    pub description: Option<String>,
    pub streams: Vec<Stream>,
    pub height: i32,
    pub width: i32,
}

pub struct Stream {
    pub url: String,
    pub quality: String,
    pub mime_type: mime::MediaType,
}
