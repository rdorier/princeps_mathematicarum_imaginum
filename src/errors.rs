use filters::Error as FilterError;

pub enum Error {
    FailedFilter(FilterError),
    InternalError(anyhow::Error),
    NotEnoughArguments,
    UnknownFilter(String),
    UnknownOperation(String),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::InternalError(err)
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedFilter(err) => std::fmt::Debug::fmt(err, f), // propagate display format to internal error type
            Error::InternalError(err) => std::fmt::Debug::fmt(err, f), // propagate display format to internal error type
            Error::NotEnoughArguments => write!(f, "Not enough arguments provided"),
            Error::UnknownFilter(filter_name) => write!(f, "Unknown filter name : {filter_name}"),
            Error::UnknownOperation(operation) => {
                write!(f, "Unknown requested operation : {operation}")
            }
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FailedFilter(err) => std::fmt::Debug::fmt(err, f), // propagate display format to internal error type
            Error::InternalError(err) => std::fmt::Display::fmt(err, f), // propagate display format to internal error type
            Error::NotEnoughArguments => write!(f, "Not enough arguments provided"),
            Error::UnknownFilter(filter_name) => write!(f, "Unknown filter name : {filter_name}"),
            Error::UnknownOperation(operation) => {
                write!(f, "Unknown requested operation : {operation}")
            }
        }
    }
}

impl std::error::Error for Error {}
