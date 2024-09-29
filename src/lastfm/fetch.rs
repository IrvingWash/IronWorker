use serde::Deserialize;

use crate::utils::{self, RequestMethod};

use super::requests_environment::RequestMetaInfo;

pub fn fetch<T>(request_meta_info: RequestMetaInfo) -> Result<T, String>
where
    T: for<'a> Deserialize<'a>,
{
    let response = reqwest::blocking::Client::new()
        .request(
            to_method(request_meta_info.method),
            request_meta_info.url.href(),
        )
        .send()
        .map_err(utils::error_to_string)?;

    let text = response.text().map_err(utils::error_to_string)?;

    serde_json::from_str::<T>(&text).map_err(utils::error_to_string)
}

fn to_method(method: RequestMethod) -> reqwest::Method {
    match method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Delete => reqwest::Method::DELETE,
        RequestMethod::Patch => reqwest::Method::PATCH,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
    }
}
