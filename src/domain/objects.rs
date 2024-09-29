#[derive(Debug)]
pub struct RecentTrack {
    pub artist_name: String,
    pub title: String,
    pub last_fm_url: String,
    pub album_title: String,
    pub date: String,
    pub last_fm_image_url: Option<String>,
}
