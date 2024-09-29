use serde::Deserialize;

use crate::utils::{self, RequestMethod};

use super::requests_environment::RequestMetaInfo;

const ERROR_PREDICATE: &str = "LastFM fetch";

pub fn fetch<T>(request_meta_info: RequestMetaInfo) -> Result<T, String>
where
    T: for<'a> Deserialize<'a>,
{
    let builder = reqwest::blocking::Client::new().request(
        to_method(&request_meta_info.method),
        request_meta_info.url.href(),
    );

    let builder = if request_meta_info.method == RequestMethod::Post {
        builder.header("Content-Length", 5)
    } else {
        builder
    };

    let response = builder
        .send()
        .map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;

    let text = response
        .text()
        .map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))?;

    serde_json::from_str::<T>(&text).map_err(|e| utils::error_to_string(e, ERROR_PREDICATE))
}

fn to_method(method: &RequestMethod) -> reqwest::Method {
    match method {
        RequestMethod::Get => reqwest::Method::GET,
        RequestMethod::Delete => reqwest::Method::DELETE,
        RequestMethod::Patch => reqwest::Method::PATCH,
        RequestMethod::Post => reqwest::Method::POST,
        RequestMethod::Put => reqwest::Method::PUT,
    }
}
