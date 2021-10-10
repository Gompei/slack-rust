use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(surf::Error),
    OpenConnectionError(Option<String>),
    UrlParseError(url::ParseError),
    IOError(std::io::Error),
    WebSocketError(async_tungstenite::tungstenite::Error),
    SerdeJsonError(serde_json::Error),
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

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(err: async_tungstenite::tungstenite::error::Error) -> Error {
        Error::WebSocketError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJsonError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::Http(ref e) => write!(f, "Http Request Error: {}", e),
            Error::OpenConnectionError(ref e) => {
                write!(f, "Connections Open API Error: {:?}", e)
            }
            Error::UrlParseError(ref e) => {
                write!(f, "Url Parse Error: {:?}", e)
            }
            Error::IOError(ref e) => {
                write!(f, "IO Error: {:?}", e)
            }
            Error::WebSocketError(ref e) => {
                write!(f, "WebSocket Error: {:?}", e)
            }
            Error::SerdeJsonError(ref e) => {
                write!(f, "SerdeJson Error: {:?}", e)
            }
        }
    }
}
