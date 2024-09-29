use std::rc::Rc;

use super::{
    fetch,
    payloads::LastFMScrobbleTrackPayload,
    responses::{LastFMAlbumInfoResponse, LastFMRecentTracksResponse, LastFMScrobbleTrackResponse},
    RequestsEnvironment,
};

pub struct Transport<'a> {
    requests_environment: Rc<RequestsEnvironment<'a>>,
}

impl<'a> Transport<'a> {
    pub fn new(requests_environment: Rc<RequestsEnvironment<'a>>) -> Self {
        Self {
            requests_environment,
        }
    }

    pub fn user_get_recent_tracks(&self, user: &str) -> Result<LastFMRecentTracksResponse, String> {
        fetch::<LastFMRecentTracksResponse>(self.requests_environment.user_get_recent_tracks(user))
    }

    pub fn album_get_info(
        &self,
        artist: &str,
        album: &str,
    ) -> Result<LastFMAlbumInfoResponse, String> {
        fetch::<LastFMAlbumInfoResponse>(self.requests_environment.album_get_info(artist, album))
    }

    pub fn scrobble_track(
        &self,
        payload: LastFMScrobbleTrackPayload,
    ) -> Result<LastFMScrobbleTrackResponse, String> {
        fetch::<LastFMScrobbleTrackResponse>(self.requests_environment.track_scrobble(payload)?)
    }
}
