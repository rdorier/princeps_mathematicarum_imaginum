use crate::{Error, Filtering};
use image::{Pixel, Rgba, RgbaImage};
use std::f64::consts::PI;

/// Filter to blur an image using the Gaussian function
/// Use separability property of the gaussian function to apply a 1-dimensional kernel on both dimensions :
/// G(x,y) = G(x) . G(y)
pub struct GaussianBlur {
    kernel: Vec<f64>,
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

        // one-dimensional kernel that will be applied both on row and on column of the image to blur
        let mut kernel = vec![0.0; kernel_size];

        for x in 0..kernel_size {
            // 2D Gaussian function: G(x, y) = (1 / (2 * pi * sigma^2)) * exp(-(x^2 + y^2) / (2 * sigma^2))
            // is transformed in a 1-dimensional function
            kernel[x] = (1.0 / (2.0 * PI * sigma * sigma))
                * (-((x * x) as f64) / (2.0 * sigma * sigma)).exp();
        }

        Ok(Self {
            kernel,
            kernel_size,
        })
    }
}

impl Filtering for GaussianBlur {
    fn filter(&self, input_img: RgbaImage) -> RgbaImage {
        let (width, height) = input_img.dimensions();

        let half_kernel_size = self.kernel_size / 2; // use to center kernel on pixel

        let mut temp_image = RgbaImage::new(width, height);
        let mut blurred_image = RgbaImage::new(width, height);

        // TODO : mutualize code to avoid similar doouble loop ? For example,s omething like :
        // let mut blurred_image = apply_1d_kernel(input_img, x)
        // blurred_image = apply_1d_kernel(blurred_image, y)

        // apply kernel horizontaly
        for y in 0..height {
            for x in 0..width {
                let mut blurred_pixel: Rgba<u8> = Rgba([0; 4]);

                for channel_index in 0..4 {
                    let mut gaussian_sum: f64 = 0.0;
                    let mut accumulated_weight = 0.0;

                    for kx in 0..self.kernel_size {
                        // calculate coordinates of current neighbour pixel (shifted by hald kernel size to center align Kernel with current pixel calculated (x,y))
                        let neighbour_x = (x as i128) + (kx as i128) - (half_kernel_size as i128);

                        if neighbour_x >= 0 && neighbour_x < width as i128 {
                            let weight = self.kernel[kx];

                            gaussian_sum += (input_img
                                .get_pixel_checked(neighbour_x as u32, y as u32)
                                .unwrap()
                                .channels()[channel_index]
                                as f64)
                                * weight;
                            accumulated_weight += weight;
                        }
                    }

                    // normalize weight to avoid having darker or brighter image
                    let normalized_gaussian_value = if accumulated_weight > 0.0 {
                        gaussian_sum / accumulated_weight
                    } else {
                        0.0
                    };
                    blurred_pixel[channel_index] = normalized_gaussian_value as u8;
                }

                temp_image[(x, y)] = blurred_pixel;
            }
        }

        // apply kernel vertically
        for y in 0..height {
            for x in 0..width {
                let mut blurred_pixel: Rgba<u8> = Rgba([0; 4]);

                for channel_index in 0..4 {
                    let mut gaussian_sum: f64 = 0.0;
                    let mut accumulated_weight = 0.0;

                    for ky in 0..self.kernel_size {
                        // calculate coordinates of current neighbour pixel (shifted by hald kernel size to center align Kernel with current pixel calculated (x,y))
                        let neighbour_y = (y as i128) + (ky as i128) - (half_kernel_size as i128);

                        if neighbour_y >= 0 && neighbour_y < height as i128 {
                            let weight = self.kernel[ky];

                            gaussian_sum += (temp_image
                                .get_pixel_checked(x as u32, neighbour_y as u32)
                                .unwrap()
                                .channels()[channel_index]
                                as f64)
                                * weight;
                            accumulated_weight += weight;
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
        let gaussian_blur_filter = GaussianBlur::try_new(0.8);
        assert!(gaussian_blur_filter.is_ok());
        assert_eq!(gaussian_blur_filter.unwrap().kernel_size, 5);
    }

    #[test]
    fn should_use_safety_check_for_kernel_size() {
        // ensure that even with very little sigma, kernel size is at least 3
        let gaussian_blur_filter = GaussianBlur::try_new(0.01);
        assert_eq!(gaussian_blur_filter.unwrap().kernel_size, 3);
    }

    #[test]
    fn should_return_odd_size() {
        // ensure that even with sigma value that would give an even number after calculation, kernel size is a odd number
        let gaussian_blur_filter = GaussianBlur::try_new(0.57);
        assert_eq!(gaussian_blur_filter.unwrap().kernel_size, 5);
    }

    #[test]
    fn should_throw_invalid_sigma() {
        let gaussian_blur_filter = GaussianBlur::try_new(-0.1);
        assert!(matches!(gaussian_blur_filter, Err(Error::InvalidSigma(_))));
    }
}
