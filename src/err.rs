use reqwest;

#[derive(Debug)]
#[cfg_attr(feature = "non_exhaustive", non_exhaustive)]
pub enum ErrorKind {
    NoProjectDir,
    ReqwestError(reqwest::Error),
    StdError(Box<dyn std::error::Error>),
    StdIoError(std::io::Error),
    UrlError(reqwest::UrlError),
}

impl From<reqwest::Error> for ErrorKind {
    fn from(e: reqwest::Error) -> Self {
        ErrorKind::ReqwestError(e)
    }
}

impl From<Box<dyn std::error::Error>> for ErrorKind {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        ErrorKind::StdError(e)
    }
}

impl From<reqwest::UrlError> for ErrorKind {
    fn from(e: reqwest::UrlError) -> Self {
        ErrorKind::UrlError(e)
    }
}
impl From<std::io::Error> for ErrorKind {
    fn from(e: std::io::Error) -> Self {
        ErrorKind::StdIoError(e)
    }
}
