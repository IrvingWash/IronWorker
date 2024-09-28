use super::lastfm_objects::LastFMBoolean;

pub struct LastFMScrobbleTrackPayload {
    artist: String,
    track: String,
    timestamp: Option<i64>,
    // TODO: trackNumber
    track_number: Option<u64>,
    mbid: Option<String>,
    album: Option<String>,
}

pub struct LastFMGetAlbumInfoPayload {
    artist: String,
    album: String,
    autocorrect: LastFMBoolean,
}
