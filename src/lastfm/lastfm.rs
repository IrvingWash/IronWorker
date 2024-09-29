use std::rc::Rc;

use super::{AuthProvider, RequestsEnvironment};

const API_KEY: &str = "8c60466b983cecc92ea98f5113a8a500";
const SHARED_SECRET: &str = "9e044c244ebfb1bbfa4e9cc56c1a01d5";
const BASE_URL: &str = "http://ws.audioscrobbler.com/2.0/";

pub struct LastFM {
    auth_provider: AuthProvider,
}

impl LastFM {
    pub fn new() -> Self {
        let requests_environment = Rc::new(RequestsEnvironment::new(BASE_URL, API_KEY));

        Self {
            auth_provider: AuthProvider::new(requests_environment.clone()),
        }
    }

    pub fn sign_in(&self) -> Result<(), String> {
        self.auth_provider.sign_in()
    }
}
