use blacksmith::{
    cli::{cli::Cli, Args},
    storage::Storage,
    LastFM,
};
use clap::Parser;

fn main() -> Result<(), String> {
    let args = Args::parse();
    let lastfm = LastFM::new();
    let storage = Storage::new()?;

    let mut cli = Cli::new(lastfm, storage, args);

    cli.start()?;

    Ok(())
}
