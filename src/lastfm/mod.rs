pub use lastfm::LastFM;
pub mod lastfm_objects;
pub mod lastfm_payloads;
pub mod lastfm_responses;

pub(self) use auth_provider::AuthProvider;
mod auth_provider;
mod lastfm;
