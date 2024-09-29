use std::rc::Rc;

use super::{
    fetch,
    lastfm_objects::LastFMSession,
    lastfm_responses::{LastFMGetTokenResponse, LastFMSessionResponse},
    RequestsEnvironment,
};

pub struct AuthProvider<'a> {
    requests_environment: Rc<RequestsEnvironment<'a>>,
    token: Option<String>,
}

impl<'a> AuthProvider<'a> {
    pub fn new(requests_environment: Rc<RequestsEnvironment<'a>>) -> Self {
        Self {
            requests_environment,
            token: None,
        }
    }

    pub fn get_token(&mut self) -> Result<String, String> {
        let meta_info = self.requests_environment.auth_get_token();

        let token = fetch::<LastFMGetTokenResponse>(meta_info)?;

        self.token = Some(token.token.clone());

        Ok(token.token)
    }

    pub fn get_session(&self) -> Result<LastFMSession, String> {
        match &self.token {
            Some(token) => {
                let session = fetch::<LastFMSessionResponse>(
                    self.requests_environment.auth_get_session(token),
                )?;

                Ok(session.session)
            }
            None => Err("went wrong".to_owned()),
        }
    }
}
