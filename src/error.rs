use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug, PartialEq)]
pub enum ScrapeError {
    InvalidResponse,
    InvalidURI,
    NoMatch,
    RequestFailed,
    RequestTimeout,
    SendError,
}

impl ScrapeError {
    pub fn message(&self) -> &str {
        match self {
            Self::InvalidResponse => "INVALID RESPONSE",
            Self::InvalidURI => "INVALID URI",
            Self::NoMatch => "NO MATCH",
            Self::RequestFailed => "REQUEST FAILED",
            Self::RequestTimeout => "REQUEST TIMEOUT",
            Self::SendError => "SEND ERROR",
        }
    }
}

impl Display for ScrapeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.message())
    }
}
