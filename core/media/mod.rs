pub mod audio;
pub mod audio_playlist;
pub mod image;
pub mod image_album;
pub mod media_playlist;
pub mod user;
pub mod video;
pub mod video_playlist;

pub enum Media {
    Video(video::Video),
    Audio(audio::Audio),
    Image(image::Image),
    VideoPlaylist(video_playlist::VideoPlaylist),
    AudioPlaylist(audio_playlist::AudioPlaylist),
    ImageAlbum(image_album::ImageAlbum),
    MediaPlaylist(media_playlist::MediaPlaylist),
    User(user::User)
}
