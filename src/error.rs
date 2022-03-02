use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
/// Fetch error
pub enum Error {
    /// Http error
    Http(reqwest::Error),
    /// Json serialization/deserialization error
    Json(serde_json::Error),
}

impl Error {
    fn description(&self) -> &str {
        match &*self {
            Error::Http(_) => "Http error on fetching algolia api",
            Error::Json(_) => "Json serialization/deserialization error on fetching algolia api",
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Error {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())?;
        match self {
            Error::Http(e) => write!(f, "{}", e),
            Error::Json(e) => write!(f, "{}", e),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Http(e) => e.source(),
            Error::Json(e) => e.source(),
        }
    }
}
