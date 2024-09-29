use std::io;

use crate::{
    domain::objects::{AlbumInfo, RecentTrack},
    storage::storage::StorageContent,
    utils, LastFM, Storage,
};

use super::{args::Commands, Args};

pub struct Cli<'a> {
    lastfm: LastFM<'a>,
    storage: Storage,
    args: Args,
}

impl<'a> Cli<'a> {
    pub fn new(lastfm: LastFM<'a>, storage: Storage, args: Args) -> Self {
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
            Commands::Scrobble { artist, album } => Ok(()),
            Commands::Album { artist, album } => self.album_info(artist, album),
        }
    }

    fn list(&mut self) -> Result<(), String> {
        let recent_tracks = self.lastfm.recent_tracks(&self.storage.load()?.username)?;

        for track in recent_tracks.iter().rev() {
            Cli::print_track(track);
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
