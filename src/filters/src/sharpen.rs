use image::{Pixel, Rgba, RgbaImage};
use image_processing::{add, substract};

use crate::{Error, Filtering, GaussianBlur};

#[derive(Debug)]
pub enum SharpenAlgorithm {
    DefaultKernel,
    Laplacian,
    UnsharpMasking,
}

impl From<&str> for SharpenAlgorithm {
    fn from(value: &str) -> Self {
        match value {
            "laplacian" => SharpenAlgorithm::Laplacian,
            "usm" => SharpenAlgorithm::UnsharpMasking,
            _ => SharpenAlgorithm::DefaultKernel,
        }
    }
}

pub struct Sharpen {
    kernel: Vec<Vec<f64>>,
    algorithm: SharpenAlgorithm,
}

impl Sharpen {
    pub fn new(sharpen_type: Option<SharpenAlgorithm>) -> Self {
        let algorithm = sharpen_type.unwrap_or(SharpenAlgorithm::DefaultKernel);
        let kernel = match algorithm {
            SharpenAlgorithm::DefaultKernel => {
                // provide default kernel to sharpen images
                vec![vec![0., -1., 0.], vec![-1., 5., -1.], vec![0., -1., 0.]]
            }
            // use laplacian operator to sharpen image
            SharpenAlgorithm::Laplacian => {
                vec![vec![0., -1., 0.], vec![-1., 4., -1.], vec![0., -1., 0.]]
            }
            SharpenAlgorithm::UnsharpMasking => vec![vec![]], // Unsharp Masking doesn't use a kernel
        };
        Self { kernel, algorithm }
    }

    fn convolution(&self, img: &RgbaImage) -> RgbaImage {
        let (width, height) = img.dimensions();

        let mut sharpen_img = RgbaImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                let mut sharpen_pixel: Rgba<u8> = Rgba([0; 4]);

                for channel_index in 0..4 {
                    let mut kernel_sum = 0.0;

                    for ky in 0..3 {
                        for kx in 0..3 {
                            let neighbour_x = (x as i128) + (kx as i128) - 1; // minus 1 to center kernel on pixel
                            let neighbour_y = (y as i128) + (ky as i128) - 1; // minus 1 to center kernel on pixel

                            if neighbour_x >= 0
                                && neighbour_x < width as i128
                                && neighbour_y >= 0
                                && neighbour_y < height as i128
                            {
                                let weight = self.kernel[ky][kx];
                                let pixel_channel_value = img
                                    .get_pixel(neighbour_x as u32, neighbour_y as u32)
                                    .channels()[channel_index];
                                kernel_sum += (pixel_channel_value as f64) * weight;
                            }
                        }
                    }

                    sharpen_pixel[channel_index] = kernel_sum as u8;
                }

                sharpen_img[(x, y)] = sharpen_pixel;
            }
        }

        sharpen_img
    }

    fn unsharp_masking(&self, img: &RgbaImage) -> Result<RgbaImage, Error> {
        // create a blur version of the original image
        let gaussian_blur_algo = GaussianBlur::try_new(3.0).unwrap(); // SAFETY sigma value greater than 0
        let blurred_img = gaussian_blur_algo.filter(img.clone())?;

        // substract blurred image from the original to get mask (which contains only high-frequency details/edges)
        let mask = substract(img, &blurred_img).unwrap(); // SAFETY blurred image created from original has same dimension

        // add mask with a scale to enforce high-frequency details/edges
        // TODO : add scaled with optional scale parameter ?
        let sharpen_image = add(img, &mask).unwrap(); // SAFETY mask image has same dimension than original image
        Ok(sharpen_image)
    }
}

impl Filtering for Sharpen {
    fn filter(&self, img: RgbaImage) -> Result<RgbaImage, Error> {
        match self.algorithm {
            SharpenAlgorithm::DefaultKernel => {
                let sharpen_img = self.convolution(&img);
                Ok(sharpen_img)
            }
            SharpenAlgorithm::Laplacian => {
                let mut sharpen_img = self.convolution(&img);
                sharpen_img = add(&img, &sharpen_img).unwrap();
                Ok(sharpen_img)
            }
            SharpenAlgorithm::UnsharpMasking => self.unsharp_masking(&img),
        }
    }
}
