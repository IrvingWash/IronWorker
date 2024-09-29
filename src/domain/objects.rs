#[derive(Debug)]
pub struct RecentTrack {
    pub artist_name: Option<String>,
    pub title: String,
    pub last_fm_url: String,
    pub album_title: String,
    pub timestamp: String,
    pub last_fm_image_url: Option<String>,
}
