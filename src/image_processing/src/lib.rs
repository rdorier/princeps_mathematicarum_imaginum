use image::{ImageReader, Pixel, Rgba, RgbaImage};
use ndarray::Array2;

mod errors;

pub use errors::*;

pub fn read_image_from_path(file_path: &String) -> Result<RgbaImage, Error> {
    // try to open input image
    let reader = ImageReader::open(file_path)
        .map_err(|_| Error::FileOpeningError(file_path.clone()))?
        .with_guessed_format()
        .map_err(|_| Error::UnknownFileFormat(file_path.clone()))?;

    // decode input image and convert it to RGBA
    let img = reader.decode().map_err(|_| Error::FileDecode)?;
    Ok(img.to_rgba8())
}

pub fn rgba_to_gray_array_conversion(img: &RgbaImage) -> Array2<f32> {
    // transform image as grayscale using coef from UIT-R BT.601-7 norm to mimic human perception

    // RGB coef from BT.601-7 norm
    let green_sensibility_coef = 0.299;
    let red_sensibility_coef = 0.587;
    let blue_sensibility_coef = 0.114;

    let (width, height) = img.dimensions();
    let mut gray_img = Array2::<f32>::zeros((height as usize, width as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b, _a] = pixel.0;
        let gray_value = green_sensibility_coef * (r as f32)
            + red_sensibility_coef * (g as f32)
            + blue_sensibility_coef * (b as f32);
        gray_img[[y as usize, x as usize]] = gray_value;
    }

    gray_img
}

pub fn gray_array_to_rgba_conversion(gray_array: &Array2<f32>) -> RgbaImage {
    // convert gray array representing pixels intensity as a RGBA image (in grayscale)

    // get array dimensions
    let (height, width) = gray_array.dim();
    // create an image with same height and widht than gray array
    let mut rgba_img = RgbaImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            // get intensity value and transfer it equally between red, blue and green channel
            let intensity = gray_array[[y, x]].clamp(0.0, 255.0) as u8;
            rgba_img.put_pixel(
                x as u32,
                y as u32,
                Rgba([intensity, intensity, intensity, 255]),
            );
        }
    }

    rgba_img
}

pub fn inverse(mut img: RgbaImage) -> RgbaImage {
    // Invert pixels value of the input image.
    for pixel in img.pixels_mut() {
        pixel.invert();
    }

    img
}

/// Correct gamma curve of the given image
pub fn gamma_correction(img: &RgbaImage, gamma: f64) -> Result<RgbaImage, Error> {
    // gamma must be greater than 0
    if gamma <= 0.0 {
        return Err(Error::NegativeOrNullGamma);
    }
    let (width, height) = img.dimensions();
    let mut corrected_img = RgbaImage::new(width, height);

    // compute gamma correction to apply
    let gamma_correction = 1.0 / gamma;

    for (x,y, pixel) in img.enumerate_pixels() {
        let gamma_corrected_pixel = pixel.map(|channel_value| (255.0 * (channel_value as f64 / 255.0).powf(gamma_correction)) as u8);

        corrected_img[(x, y)] = gamma_corrected_pixel;
    }

    Ok(corrected_img)
}
