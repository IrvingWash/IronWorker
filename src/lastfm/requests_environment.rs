use crate::utils::{RequestMethod, URL};

use super::CallSigner;

pub struct RequestsEnvironment<'a> {
    base_url: &'a str,
    api_key: &'a str,
    call_singer: CallSigner<'a>,
}

pub struct RequestMetaInfo {
    pub url: URL,
    pub method: RequestMethod,
}

impl<'a> RequestsEnvironment<'a> {
    pub fn new(base_url: &'a str, api_key: &'a str, call_singer: CallSigner<'a>) -> Self {
        Self {
            base_url,
            api_key,
            call_singer,
        }
    }

    pub fn auth_get_token(&self) -> RequestMetaInfo {
        let mut url = URL::new(&self.base_url).unwrap();

        url.add_query_param("method", "auth.getToken");
        url.add_query_param("api_key", &self.api_key);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    pub fn auth_get_session(&self, token: &str) -> RequestMetaInfo {
        let mut url = URL::new(&self.base_url).unwrap();

        url.add_query_param("method", "auth.getSession");
        url.add_query_param("api_key", &self.api_key);
        url.add_query_param("token", token);

        self.add_signature(&mut url);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    fn add_signature(&self, url: &mut URL) {
        url.add_query_param("api_sig", &self.call_singer.sign(url.query_params()));
    }

    fn add_common_query_params(url: &mut URL) {
        url.add_query_param("format", "json");
    }
}
