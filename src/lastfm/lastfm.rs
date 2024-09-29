use super::AuthProvider;

const API_KEY: &str = "8c60466b983cecc92ea98f5113a8a500";
const SHARED_SECRET: &str = "9e044c244ebfb1bbfa4e9cc56c1a01d5";

pub struct LastFM {
    auth_provider: AuthProvider,
}
