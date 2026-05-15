use image::RgbaImage;

pub mod gaussian_blur;

pub use gaussian_blur::*;

/// Trait describing fitlers behaviour
pub trait Filtering {
    /// Filter input image
    /// 
    /// # Arguments
    /// - `img` - The input image to filter.
    /// 
    /// # Return
    /// A new image, resulting of the applied filter to input image data.
    fn filter(&self, img: RgbaImage) -> RgbaImage;
}
