use super::lastfm_objects::{
    LastFMAlbumInfoResponseAlbum, LastFMRecentTracks, LastFMScrobbleTrackResponseAttr,
    LastFMSession,
};

pub struct LastFMSessionResponse {
    session: LastFMSession,
}

pub struct LastFMRecentTracksResponse {
    // TODO: recenttracks
    recent_tracks: LastFMRecentTracks,
}

pub struct LastFMScrobbleTrackResponse {
    // TODO: @attr
    attr: LastFMScrobbleTrackResponseAttr,
}

pub struct LastFMAlbumInfoResponse {
    album: LastFMAlbumInfoResponseAlbum,
}
