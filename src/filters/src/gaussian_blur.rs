use crate::{Error, Filtering};
use image::{Pixel, Rgba, RgbaImage};
use std::f64::consts::PI;

/// Filter to blur an image using the Gaussian function
pub struct GaussianBlur {
    kernel: Vec<Vec<f64>>,
    kernel_size: usize,
}

impl GaussianBlur {
    /// Create a new gaussian blur filter.
    ///
    /// # Arguments
    /// * `sigma` - The standard deviation controlling the blur spread.
    ///
    /// # Invariant
    /// - `sigma` value must be > 0
    pub fn try_new(sigma: f64) -> Result<Self, Error> {
        if sigma < 0.0 {
            return Err(Error::InvalidSigma(sigma));
        }

        let mut kernel_size = ((6.0 * sigma).ceil() as usize).max(3);
        if kernel_size.is_multiple_of(2) {
            kernel_size += 1
        };

        // row major matrix representing gaussian kernel to apply to pixels : first index represents the row, second one the columns
        let mut kernel = vec![vec![0.0; kernel_size]; kernel_size];

        for y in 0..kernel_size {
            for x in 0..kernel_size {
                // 2D Gaussian function: G(x, y) = (1 / (2 * pi * sigma^2)) * exp(-(x^2 + y^2) / (2 * sigma^2))
                kernel[y][x] = (1.0 / (2.0 * PI * sigma * sigma))
                    * (-((x * x + y * y) as f64) / (2.0 * sigma * sigma)).exp();
            }
        }

        Ok(Self {
            kernel,
            kernel_size,
        })
    }
}

impl Filtering for GaussianBlur {
    fn filter(&self, input_img: RgbaImage) -> RgbaImage {
        // TODO : use separability property of the gaussian function to apply two 1-dimensional (one on rows, the other one on columns)

        let (width, height) = input_img.dimensions();

        let mut blurred_image = RgbaImage::new(width, height);

        for y in 1..height {
            for x in 1..width {
                let half_kernel_size = self.kernel_size / 2; // use to center kernel on pixel

                let mut blurred_pixel: Rgba<u8> = Rgba([0; 4]);

                for channel_index in 0..4 {
                    let mut gaussian_sum: f64 = 0.0;
                    let mut accumulated_weight = 0.0;

                    for ky in 0..self.kernel_size {
                        for kx in 0..self.kernel_size {
                            // calculate coordinates of current neighbour pixel (shifted by hald kernel size to center align Kernel with current pixel calculated (x,y))
                            let neighbour_x =
                                (x as i128) + (kx as i128) - (half_kernel_size as i128);
                            let neighbour_y =
                                (y as i128) + (ky as i128) - (half_kernel_size as i128);

                            if neighbour_x >= 0
                                && neighbour_x < width as i128
                                && neighbour_y >= 0
                                && neighbour_y < height as i128
                            {
                                let weight = self.kernel[ky][kx];

                                gaussian_sum += (input_img
                                    .get_pixel_checked(neighbour_x as u32, neighbour_y as u32)
                                    .unwrap()
                                    .channels()[channel_index]
                                    as f64)
                                    * weight;
                                accumulated_weight += weight;
                            }
                        }
                    }

                    // normalize weight to avoid having adrker or brighter image
                    let normalized_gaussian_value = if accumulated_weight > 0.0 {
                        gaussian_sum / accumulated_weight
                    } else {
                        0.0
                    };
                    blurred_pixel[channel_index] = normalized_gaussian_value as u8;
                }

                blurred_image[(x, y)] = blurred_pixel;
            }
        }

        blurred_image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_compute_gaussian_kernel() {
        let gaussian_blur_filter = GaussianBlur::try_new(0.1);
        assert!(gaussian_blur_filter.is_ok());
    }

    #[test]
    fn should_throw_invalid_sigma() {
        let gaussian_blur_filter = GaussianBlur::try_new(-0.1);
        assert!(matches!(gaussian_blur_filter, Err(Error::InvalidSigma(_))));
    }
}
