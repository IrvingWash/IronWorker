use std::rc::Rc;

use super::{fetch, responses::LastFMRecentTracksResponse, RequestsEnvironment};

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
}
