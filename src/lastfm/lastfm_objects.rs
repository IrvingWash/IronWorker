use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LastFMSession {
    key: String,
    name: String,
}

pub struct LastFMRecentTracks {
    // TODO: track
    tracks: Vec<LastFMRecentTrack>,
}

pub struct LastFMRecentTrack {
    album: LastFMRecentTrackAlbum,
    artist: LastFMRecentTrackArtist,
    date: LastFMRecentTrackDate,
    // TODO: image
    images: Vec<LastFMImage>,
    mbid: String,
    name: String,
    url: String,
    loved: LastFMBoolean,
}

pub struct LastFMRecentTrackAlbum {
    // TODO: #text
    text: String,
    mbid: String,
}

pub struct LastFMRecentTrackArtist {
    name: String,
    mbid: String,
}

pub struct LastFMRecentTrackDate {
    // TODO: #text
    text: String,
    // TODO: uts
    utc: String,
}

pub struct LastFMImage {
    // TODO: #text
    text: String,
    size: LastFMImageSize,
}

pub enum LastFMImageSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
    Mega,
}

impl LastFMImageSize {
    pub fn value(&self) -> &str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
            Self::ExtraLarge => "extralarge",
            Self::Mega => "mega",
        }
    }
}

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
