#[derive(Debug)]
pub enum Error {
    InvalidSigma(f64),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidSigma(value) => write!(
                f,
                "Sigma value must be greater than 0. Value providen was {value}"
            ),
        }
    }
}

impl std::error::Error for Error {}
