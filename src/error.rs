use std::fmt;

#[derive(Debug)]
pub enum Error {
    InvalidInputError,
    IOError(std::io::Error),
    NotFoundDomain,
    NotFoundStream,
    SerdeJsonError(serde_json::Error),
    SocketModeOpenConnectionError,
    SurfError(surf::Error),
    UrlParseError(url::ParseError),
    WebSocketError(async_tungstenite::tungstenite::Error),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IOError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::SerdeJsonError(err)
    }
}

impl From<surf::Error> for Error {
    fn from(err: surf::Error) -> Error {
        Error::SurfError(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Error {
        Error::UrlParseError(err)
    }
}

impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(err: async_tungstenite::tungstenite::error::Error) -> Error {
        Error::WebSocketError(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::InvalidInputError => write!(f, "Invalid Input Error"),
            Error::IOError(ref e) => write!(f, "IO Error: {}", e),
            Error::NotFoundDomain => write!(f, "NotFound Domain Error"),
            Error::NotFoundStream => write!(f, "NotFound Stream Error"),
            Error::SerdeJsonError(ref e) => write!(f, "Serde Json Error: {}", e),
            Error::SocketModeOpenConnectionError => write!(f, "SocketMode OpenConnection Error"),
            Error::SurfError(ref e) => write!(f, "Surf Error: {}", e),
            Error::UrlParseError(ref e) => write!(f, "Url Parse Error: {}", e),
            Error::WebSocketError(ref e) => write!(f, "WebSocket Error: {:?}", e),
        }
    }
}
