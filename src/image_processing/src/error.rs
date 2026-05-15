#[derive(Debug)]
pub enum Error {
    FileDecode,
    FileOpeningError(String),
    UnknownFileFormat(String),
}