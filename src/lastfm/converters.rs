use crate::domain::objects::{
    AlbumInfo, RecentTrack, ScrobbleTrackPayload, Track, TrackScrobblingResult,
};

use super::{
    objects::{
        LastFMAlbumInfoResponseAlbum, LastFMAlbumInfoResponseAlbumTracksTrack, LastFMImage,
        LastFMImageSize, LastFMRecentTrack,
    },
    payloads::LastFMScrobbleTrackPayload,
    responses::LastFMScrobbleTrackResponse,
};

pub fn convert_lastfm_scrobbling_result(
    lastfm_scrobbling_result: LastFMScrobbleTrackResponse,
) -> TrackScrobblingResult {
    TrackScrobblingResult {
        accepted: lastfm_scrobbling_result.scrobbles.attribute.accepted == 1,
    }
}

pub fn convert_scrobble_track_payload(
    scrobble_track_payload: ScrobbleTrackPayload,
) -> LastFMScrobbleTrackPayload {
    LastFMScrobbleTrackPayload {
        album: scrobble_track_payload.album_title,
        artist: scrobble_track_payload.artist_name,
        timestamp: scrobble_track_payload.timestamp,
        track: scrobble_track_payload.track_title,
        track_number: scrobble_track_payload.track_number,
    }
}

pub fn convert_lastfm_album_info(last_fm_album_info: LastFMAlbumInfoResponseAlbum) -> AlbumInfo {
    AlbumInfo {
        artist_name: last_fm_album_info.artist,
        lastfm_image_url: find_large_image_url(last_fm_album_info.images),
        lastfm_url: last_fm_album_info.url,
        title: last_fm_album_info.name,
        tracks: last_fm_album_info
            .tracks
            .tracks
            .into_iter()
            .map(convert_lastfm_album_info_track)
            .collect(),
    }
}

pub fn convert_lastfm_recent_tracks(lastfm_recent_track: LastFMRecentTrack) -> RecentTrack {
    RecentTrack {
        album_title: lastfm_recent_track.album.text,
        artist_name: lastfm_recent_track.artist.name,
        lastfm_image_url: find_large_image_url(lastfm_recent_track.images),
        lastfm_url: lastfm_recent_track.url,
        title: lastfm_recent_track.name,
        date: lastfm_recent_track.date.text,
    }
}

pub fn convert_lastfm_album_info_track(
    lastfm_album_info_track: LastFMAlbumInfoResponseAlbumTracksTrack,
) -> Track {
    Track {
        artist_name: lastfm_album_info_track.artist.name,
        lastfm_url: lastfm_album_info_track.url,
        title: lastfm_album_info_track.name,
        track_number: lastfm_album_info_track.attribute.rank,
    }
}

fn find_large_image_url(images: Vec<LastFMImage>) -> Option<String> {
    images
        .iter()
        .find(|i| i.size == LastFMImageSize::Large)
        .map(|image| image.text.clone())
}
