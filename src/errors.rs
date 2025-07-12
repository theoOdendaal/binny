#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    Http(reqwest::Error),
    Stream(tungstenite::Error),
    Serde(serde_json::Error),
    Zip(zip::result::ZipError),
    Other(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(e) => write!(f, "{e}"),
            Self::Http(e) => write!(f, "{e}"),
            Self::Stream(e) => write!(f, "{e}"),
            Self::Serde(e) => write!(f, "{e}"),
            Self::Zip(e) => write!(f, "{e}"),
            Self::Other(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::Http(value)
    }
}

impl From<tungstenite::Error> for Error {
    fn from(value: tungstenite::Error) -> Self {
        Error::Stream(value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::Serde(value)
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(value: zip::result::ZipError) -> Self {
        Error::Zip(value)
    }
}
