use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(surf::Error),
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Error {
        Error::Http(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http Request Error: {}", e),
        }
    }
}
