use ironworker::lastfm::LastFM;

fn main() -> Result<(), String> {
    let lastfm = LastFM::new();

    lastfm.sign_in()
}
