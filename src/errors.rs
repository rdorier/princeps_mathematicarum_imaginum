pub enum Error {
    InternalError(anyhow::Error),
    UnknownFilter(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::InternalError(err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => std::fmt::Debug::fmt(err, f), // propagate display format to internal error type
            Error::UnknownFilter(filter_name) => write!(f, "Unknown filter name : {filter_name}"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InternalError(err) => std::fmt::Display::fmt(err, f), // propagate display format to internal error type
            Error::UnknownFilter(filter_name) => write!(f, "Unknown filter name : {filter_name}"),
        }
    }
}

impl std::error::Error for Error {}
