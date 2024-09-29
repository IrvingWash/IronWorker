use serde::Deserialize;

use super::lastfm_objects::{
    LastFMAlbumInfoResponseAlbum, LastFMRecentTracks, LastFMScrobbleTrackResponseAttr,
    LastFMSession,
};

#[derive(Deserialize, Debug)]
pub struct LastFMGetTokenResponse {
    pub token: String,
}

pub struct LastFMSessionResponse {
    pub session: LastFMSession,
}

pub struct LastFMRecentTracksResponse {
    // TODO: recenttracks
    pub recent_tracks: LastFMRecentTracks,
}

pub struct LastFMScrobbleTrackResponse {
    // TODO: @attr
    pub attr: LastFMScrobbleTrackResponseAttr,
}

pub struct LastFMAlbumInfoResponse {
    pub album: LastFMAlbumInfoResponseAlbum,
}
