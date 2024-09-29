#[derive(Debug, PartialEq)]
pub enum RequestMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

impl RequestMethod {
    pub fn value(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
        }
    }
}
