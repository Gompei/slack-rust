use std::error;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(surf::Error),
    OpenConnectionError(Option<String>),
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Error {
        Error::Http(err)
    }
}

impl From<Option<String>> for Error {
    fn from(err: Option<String>) -> Error {
        Error::OpenConnectionError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http Request Error: {}", e),
            Error::OpenConnectionError(ref e) => {
                write!(f, "Connections Open API Error: {:?}", e)
            }
        }
    }
}
