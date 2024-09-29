use std::rc::Rc;

use crate::domain::objects::RecentTrack;

use super::{
    converters::convert_lastfm_recent_tracks, objects::LastFMSession, AuthProvider, CallSigner,
    RequestsEnvironment, Transport,
};

const API_KEY: &str = "8c60466b983cecc92ea98f5113a8a500";
const SHARED_SECRET: &str = "9e044c244ebfb1bbfa4e9cc56c1a01d5";
const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct LastFM<'a> {
    auth_provider: AuthProvider<'a>,
    transport: Transport<'a>,
}

impl<'a> Default for LastFM<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> LastFM<'a> {
    pub fn new() -> Self {
        let requests_environment = Rc::new(RequestsEnvironment::new(
            BASE_URL,
            API_KEY,
            CallSigner::new(SHARED_SECRET),
        ));

        Self {
            auth_provider: AuthProvider::new(requests_environment.clone()),
            transport: Transport::new(requests_environment.clone()),
        }
    }

    pub fn request_auth(&mut self) -> Result<String, String> {
        let token = self.auth_provider.get_token()?;

        Ok(format!(
            "https://www.last.fm/api/auth/?api_key={}&token={}",
            API_KEY, token
        ))
    }

    pub fn session(&mut self) -> Result<LastFMSession, String> {
        self.auth_provider.get_session()
    }

    pub fn recent_tracks(&self, username: &str) -> Result<Vec<RecentTrack>, String> {
        let mut recent_tracks = self.transport.user_get_recent_tracks(username)?;

        Ok(recent_tracks
            .recent_tracks
            .tracks
            .drain(..)
            .map(convert_lastfm_recent_tracks)
            .collect())
    }
}
