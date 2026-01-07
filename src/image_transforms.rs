use image::{RgbaImage, Rgba, Pixel};
use ndarray::{Array2};


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
            rgba_img.put_pixel(x as u32, y as u32, Rgba([intensity, intensity, intensity, 255]));
        }
    }

    rgba_img
}



pub fn inverse(mut img : RgbaImage ) -> RgbaImage  {
    // Invert pixels value of the input image.
    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    img
}
