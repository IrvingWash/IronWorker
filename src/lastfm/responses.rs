use serde::Deserialize;

use super::objects::{
    LastFMAlbumInfoResponseAlbum, LastFMRecentTracks, LastFMScrobbleTrackResponseAttr,
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

pub struct LastFMScrobbleTrackResponse {
    // TODO: @attr
    pub attr: LastFMScrobbleTrackResponseAttr,
}

#[derive(Deserialize, Debug)]
pub struct LastFMAlbumInfoResponse {
    pub album: LastFMAlbumInfoResponseAlbum,
}
