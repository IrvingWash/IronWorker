use std::rc::Rc;

use super::{fetch, lastfm_responses::LastFMGetTokenResponse, RequestsEnvironment};

pub struct AuthProvider {
    requests_environment: Rc<RequestsEnvironment>,
}

impl AuthProvider {
    pub fn new(requests_environment: Rc<RequestsEnvironment>) -> Self {
        Self {
            requests_environment,
        }
    }

    pub fn sign_in(&self) -> Result<(), String> {
        let token = self.get_token()?;

        dbg!(token);

        Ok(())
    }

    fn get_token(&self) -> Result<LastFMGetTokenResponse, String> {
        let meta_info = self.requests_environment.auth_get_token();

        let token = fetch::<LastFMGetTokenResponse>(meta_info)?;

        Ok(token)
    }
}
