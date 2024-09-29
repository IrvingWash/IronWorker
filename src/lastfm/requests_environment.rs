use crate::{
    utils::{RequestMethod, Url},
    Storage,
};

use super::{payloads::LastFMScrobbleTrackPayload, CallSigner};

pub struct RequestsEnvironment<'a> {
    base_url: &'a str,
    api_key: &'a str,
    call_singer: CallSigner<'a>,
    storage: &'a Storage,
}

pub struct RequestMetaInfo {
    pub url: Url,
    pub method: RequestMethod,
}

impl<'a> RequestsEnvironment<'a> {
    pub fn new(
        base_url: &'a str,
        api_key: &'a str,
        call_singer: CallSigner<'a>,
        storage: &'a Storage,
    ) -> Self {
        Self {
            base_url,
            api_key,
            call_singer,
            storage,
        }
    }

    pub fn auth_get_token(&self) -> RequestMetaInfo {
        let mut url = Url::new(self.base_url).unwrap();

        url.add_query_param("method", "auth.getToken");
        url.add_query_param("api_key", self.api_key);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    pub fn auth_get_session(&self, token: &str) -> RequestMetaInfo {
        let mut url = Url::new(self.base_url).unwrap();

        url.add_query_param("method", "auth.getSession");
        url.add_query_param("api_key", self.api_key);
        url.add_query_param("token", token);

        self.add_signature(&mut url);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    pub fn user_get_recent_tracks(&self, user: &str) -> RequestMetaInfo {
        let mut url = Url::new(self.base_url).unwrap();

        url.add_query_param("method", "user.getRecentTracks");
        url.add_query_param("user", user);
        url.add_query_param("api_key", self.api_key);

        self.add_signature(&mut url);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    pub fn album_get_info(&self, artist: &str, album: &str) -> RequestMetaInfo {
        let mut url = Url::new(self.base_url).unwrap();

        url.add_query_param("method", "album.getInfo");
        url.add_query_param("artist", artist);
        url.add_query_param("album", album);
        url.add_query_param("api_key", self.api_key);

        RequestsEnvironment::add_common_query_params(&mut url);

        RequestMetaInfo {
            url,
            method: RequestMethod::Get,
        }
    }

    pub fn track_scrobble(
        &self,
        payload: LastFMScrobbleTrackPayload,
    ) -> Result<RequestMetaInfo, String> {
        let mut url = Url::new(self.base_url).unwrap();

        url.add_query_param("artist", &payload.artist);
        url.add_query_param("track", &payload.track);
        url.add_query_param("timestamp", &payload.timestamp.to_string());
        if let Some(album) = payload.album {
            url.add_query_param("album", &album);
        }
        if let Some(track_number) = payload.track_number {
            url.add_query_param("trackNumber", &track_number.to_string());
        }

        url.add_query_param("method", "track.scrobble");
        url.add_query_param("api_key", self.api_key);
        url.add_query_param("sk", &self.storage.load()?.session_key);

        self.add_signature(&mut url);

        RequestsEnvironment::add_common_query_params(&mut url);

        Ok(RequestMetaInfo {
            url,
            method: RequestMethod::Post,
        })
    }

    fn add_signature(&self, url: &mut Url) {
        url.add_query_param("api_sig", &self.call_singer.sign(url.query_params()));
    }

    fn add_common_query_params(url: &mut Url) {
        url.add_query_param("format", "json");
    }
}
