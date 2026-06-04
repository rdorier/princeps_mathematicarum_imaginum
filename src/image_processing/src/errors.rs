#[derive(Debug)]
pub enum Error {
    FileDecode,
    FileOpeningError(String),
    NegativeOrNullGamma,
    UnknownFileFormat(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FileDecode => write!(f, "Could not decode image."),
            Error::FileOpeningError(path) => write!(f, "Failed to open input file {path}"),
            Error::NegativeOrNullGamma => write!(f, "Gamma must be greater than 0"),
            Error::UnknownFileFormat(path) => write!(
                f,
                "Could not guess image format for following image : {path}"
            ),
        }
    }
}

impl std::error::Error for Error {}
