use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastFMSession {
    pub key: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTracks {
    #[serde(rename(deserialize = "track"))]
    pub tracks: Vec<LastFMRecentTrack>,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTrack {
    pub album: LastFMRecentTrackAlbum,
    pub artist: LastFMRecentTrackArtist,
    pub date: LastFMRecentTrackDate,
    #[serde(rename(deserialize = "image"))]
    pub images: Vec<LastFMImage>,
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTrackAlbum {
    #[serde(rename(deserialize = "#text"))]
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTrackArtist {
    pub name: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LastFMRecentTrackDate {
    #[serde(rename(deserialize = "uts"))]
    pub utc: String,
}

#[derive(Deserialize, Debug)]
pub struct LastFMImage {
    #[serde(rename(deserialize = "#text"))]
    pub text: String,
    pub size: LastFMImageSize,
}

#[derive(Deserialize, Debug, PartialEq)]
pub enum LastFMImageSize {
    #[serde(rename(deserialize = "small"))]
    Small,
    #[serde(rename(deserialize = "medium"))]
    Medium,
    #[serde(rename(deserialize = "large"))]
    Large,
    #[serde(rename(deserialize = "extralarge"))]
    ExtraLarge,
    #[serde(rename(deserialize = "mega"))]
    Mega,
}

#[derive(Deserialize, Debug)]
pub enum LastFMBoolean {
    True,
    False,
}

impl LastFMBoolean {
    pub fn string_value(&self) -> &str {
        match self {
            Self::True => "1",
            Self::False => "0",
        }
    }

    pub fn number_value(&self) -> u8 {
        match self {
            Self::True => 1,
            Self::False => 0,
        }
    }
}

pub struct LastFMScrobbleTrackResponseAttr {
    accepted: LastFMBoolean,
    ignored: LastFMBoolean,
}

pub struct LastFMAlbumInfoResponseAlbum {
    artist: String,
    listeners: String,
    mbid: String,
    // TODO: image
    images: Vec<LastFMImage>,
    name: String,
    // TODO: playcount
    play_count: String,
    track: LastFMAlbumInfoResponseAlbumTracks,
    url: String,
}

pub struct LastFMAlbumInfoResponseAlbumTracks {
    // TODO: track
    tracks: LastFMAlbumInfoResponseAlbumTracksTrack,
}

pub struct LastFMAlbumInfoResponseAlbumTracksTrack {
    artist: LastFMAlbumInfoResponseAlbumTracksTrackArtist,
    duration: i64,
    name: String,
    url: String,
}

pub struct LastFMAlbumInfoResponseAlbumTracksTrackArtist {
    mbid: String,
    name: String,
    url: String,
}
