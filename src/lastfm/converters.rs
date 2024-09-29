use crate::domain::objects::RecentTrack;

use super::objects::{LastFMImage, LastFMImageSize, LastFMRecentTrack};

pub fn convert_lastfm_recent_tracks(lastfm_recent_track: LastFMRecentTrack) -> RecentTrack {
    RecentTrack {
        album_title: lastfm_recent_track.album.text,
        artist_name: lastfm_recent_track.artist.name,
        last_fm_image_url: find_large_image_url(lastfm_recent_track.images),
        last_fm_url: lastfm_recent_track.url,
        timestamp: lastfm_recent_track.date.utc,
        title: lastfm_recent_track.name,
    }
}

fn find_large_image_url(images: Vec<LastFMImage>) -> Option<String> {
    images
        .iter()
        .find(|i| i.size == LastFMImageSize::Large)
        .map(|image| image.text.clone())
}
