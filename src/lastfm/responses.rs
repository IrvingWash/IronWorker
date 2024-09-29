use serde::Deserialize;

use super::objects::{
    LastFMAlbumInfoResponseAlbum, LastFMRecentTracks, LastFMScrobbleTrackResponseAttribute,
    LastFMSession,
};

#[derive(Deserialize, Debug)]
pub struct LastFMGetTokenResponse {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct LastFMSessionResponse {
    pub session: LastFMSession,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTracksResponse {
    #[serde(rename(deserialize = "recenttracks"))]
    pub recent_tracks: LastFMRecentTracks,
}

#[derive(Deserialize, Debug)]
pub struct LastFMScrobbleTrackResponse {
    pub scrobbles: LastFMScrobbleTrackResponseScrobbles,
}

#[derive(Deserialize, Debug)]
pub struct LastFMScrobbleTrackResponseScrobbles {
    #[serde(rename(deserialize = "@attr"))]
    pub attribute: LastFMScrobbleTrackResponseAttribute,
}

#[derive(Deserialize, Debug)]
pub struct LastFMAlbumInfoResponse {
    pub album: LastFMAlbumInfoResponseAlbum,
}
