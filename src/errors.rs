use image_processing::Error as ImageProcessError;

#[derive(Debug)]
pub enum Error {
    FileReadingError(ImageProcessError),
    NotEnoughArguments,
    NotANumberParameter(String),
    SavingImageFailure,
    Other(anyhow::Error),
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Other(err)
    }
}
