use std::error::Error;

pub fn error_to_string(error: impl Error, predicate: &str) -> String {
    format!("{}: {}", predicate, error.to_string())
}
