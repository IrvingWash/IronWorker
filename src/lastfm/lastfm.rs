use std::rc::Rc;

use super::{AuthProvider, CallSigner, RequestsEnvironment};

const API_KEY: &str = "8c60466b983cecc92ea98f5113a8a500";
const SHARED_SECRET: &str = "9e044c244ebfb1bbfa4e9cc56c1a01d5";
const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct LastFM<'a> {
    auth_provider: AuthProvider<'a>,
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
        }
    }

    pub fn request_auth(&mut self) -> Result<String, String> {
        let token = self.auth_provider.get_token()?;

        Ok(format!(
            "https://www.last.fm/api/auth/?api_key={}&token={}",
            API_KEY, token
        ))
    }

    pub fn get_session(&mut self) -> Result<(), String> {
        let session = self.auth_provider.get_session()?;

        dbg!(session);

        Ok(())
    }
}
