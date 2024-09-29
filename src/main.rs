use std::io;

use ironworker::lastfm::LastFM;

fn main() -> Result<(), String> {
    let mut lastfm = LastFM::new();

    let auth_url = lastfm.request_auth()?;
    println!("{}", auth_url);

    let mut ur = String::from("");
    io::stdin().read_line(&mut ur).unwrap();

    lastfm.get_session()?;

    Ok(())
}
