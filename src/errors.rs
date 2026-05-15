use image::ImageError;
use image_processing::Error as ImageProcessError;

#[derive(Debug)]
pub enum Error {
    FileReadingError(ImageProcessError),
    NotEnoughArguments,
    NotANumberParameter(String),
    SavingImageFailure(ImageError),
}