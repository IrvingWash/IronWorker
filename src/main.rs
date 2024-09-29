use std::io;

use blacksmith::{lastfm::LastFM, storage::Storage};

fn main() -> Result<(), String> {
    Storage::new();
    let mut lastfm = LastFM::new();

    let auth_url = lastfm.request_auth()?;
    println!("{}", auth_url);

    let mut ur = String::from("");
    io::stdin().read_line(&mut ur).unwrap();

    lastfm.get_session()?;

    Ok(())
}
