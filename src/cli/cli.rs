use std::io;

use crate::{storage::storage::StorageContent, utils, LastFM, Storage};

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
        }
    }

    fn list(&mut self) -> Result<(), String> {
        let recent_tracks = self.lastfm.recent_tracks(&self.storage.load()?.username)?;

        for track in recent_tracks {
            println!("=== track ===");
            println!("{:#?}", track);
            println!("=============");
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
}
