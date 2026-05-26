pub enum Error {
    NotEnoughArguments,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => std::fmt::Debug::fmt(err, f),
            Error::NotEnoughArguments => write!(f, "Not enough arguments provided"),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Other(err) => std::fmt::Display::fmt(err, f),
            Error::NotEnoughArguments => write!(f, "Not enough arguments provided"),
        }
    }
}

impl std::error::Error for Error {}
