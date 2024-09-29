use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct LastFMScrobbleTrackPayload {
    pub artist: String,
    pub track: String,
    pub album: Option<String>,
    pub timestamp: u64,
    #[serde(rename(serialize = "trackNumber"))]
    pub track_number: Option<u64>,
}
