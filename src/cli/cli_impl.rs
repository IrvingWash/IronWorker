use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    domain::objects::{AlbumInfo, RecentTrack, ScrobbleTrackPayload},
    storage::StorageContent,
    utils, LastFM, Storage,
};

use super::{args::Commands, Args};

pub struct Cli<'a> {
    lastfm: LastFM<'a>,
    storage: &'a Storage,
    args: Args,
}

impl<'a> Cli<'a> {
    pub fn new(lastfm: LastFM<'a>, storage: &'a Storage, args: Args) -> Self {
        Self {
            lastfm,
            storage,
            args,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        match &self.args.command {
            Commands::Auth => self.authenticate(),
            Commands::List => self.list(),
            Commands::Album { artist, album } => self.album_info(artist, album),
            Commands::ScrobbleTrack {
                artist,
                album,
                track,
            } => self.scrobble_track(artist, album, track, None),
            Commands::ScrobbleAlbum { artist, album } => self.scrobble_album(artist, album),
        }
    }

    fn scrobble_track(
        &self,
        artist: &str,
        album: &str,
        track: &str,
        track_number: Option<u64>,
    ) -> Result<(), String> {
        let result = self.lastfm.scrobble_track(ScrobbleTrackPayload {
            album_title: Some(album.to_owned()),
            artist_name: artist.to_owned(),
            track_title: track.to_owned(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| utils::error_to_string(e, "Getting UTC timestamp"))?
                .as_secs()
                + track_number.unwrap_or(0),
            track_number: None,
        })?;

        if !result.accepted {
            return Err(format!(
                "Failed to scrobble track: {} - {} - {}",
                artist, album, track
            ));
        }

        Ok(())
    }

    fn list(&mut self) -> Result<(), String> {
        let recent_tracks = self.lastfm.recent_tracks(&self.storage.load()?.username)?;

        for track in recent_tracks.iter().rev() {
            Cli::print_track(track);
        }

        Ok(())
    }

    fn scrobble_album(&self, artist: &str, album: &str) -> Result<(), String> {
        let album_info = self.lastfm.album_info(artist, album)?;

        for track in album_info.tracks {
            self.scrobble_track(
                &track.artist_name,
                &album_info.title,
                &track.title,
                Some(track.track_number),
            )?;
        }

        Ok(())
    }

    fn authenticate(&mut self) -> Result<(), String> {
        let auth_url = self.lastfm.request_auth()?;

        println!("go to {} to authenticate", auth_url);
        println!("press enter after you've granted access");

        let mut response = String::from("");

        io::stdin()
            .read_line(&mut response)
            .map_err(|e| utils::error_to_string(e, "Awaiting input"))?;

        let session = self.lastfm.session()?;

        self.storage.save(StorageContent {
            session_key: session.key,
            username: session.name,
        })?;

        Ok(())
    }

    fn album_info(&self, artist: &str, album: &str) -> Result<(), String> {
        let album_info = self.lastfm.album_info(artist, album)?;

        Cli::print_album_info(&album_info);

        Ok(())
    }

    fn print_track(track: &RecentTrack) {
        println!("===== track =====");
        println!("artist: {}", track.artist_name);
        println!("album: {}", track.album_title);
        println!("track: {}", track.title);
        println!("date: {}", track.date);
        println!("=================");
        println!("\n");
    }

    fn print_album_info(album_info: &AlbumInfo) {
        println!("===== album info =====");
        println!("artist: {}", album_info.artist_name);
        println!("title: {}", album_info.title);
        println!("=== tracks ===");
        for track in &album_info.tracks {
            println!("{}. {}", track.track_number, track.title);
        }
        println!("==============");
        println!("======================");
    }
}
