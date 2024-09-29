use crate::utils::{RequestMethod, URL};

pub struct RequestsEnvironment {
    base_url: String,
    api_key: String,
}

pub struct RequestMetaInfo {
    pub url: URL,
    pub method: RequestMethod,
}

impl RequestsEnvironment {
    pub fn new(base_url: &str, api_key: &str) -> Self {
        Self {
            base_url: base_url.to_owned(),
            api_key: api_key.to_owned(),
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

    fn add_common_query_params(url: &mut URL) {
        url.add_query_param("format", "json");
    }
}
