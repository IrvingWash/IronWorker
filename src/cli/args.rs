use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// authenticate to lastfm
    Auth,
    /// list recent tracks
    List,
    /// scrobble a track
    ScrobbleTrack {
        artist: String,
        album: String,
        track: String,
    },
    /// scrobble an album
    ScrobbleAlbum { artist: String, album: String },
    /// get album info
    Album { artist: String, album: String },
}

impl Default for Args {
    fn default() -> Self {
        Self::new()
    }
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
