use crate::{Error, Filtering};
use image::{Pixel, Rgba, RgbaImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
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
        let mut total_weights: f64 = 0.0;

        for x in 0..kernel_size {
            // 2D Gaussian function: G(x, y) = (1 / (2 * pi * sigma^2)) * exp(-(x^2 + y^2) / (2 * sigma^2))
            // is transformed in a 1-dimensional function
            let weight = (1.0 / (2.0 * PI * sigma * sigma))
                * (-((x * x) as f64) / (2.0 * sigma * sigma)).exp();

            kernel[x] = weight;
            total_weights += weight;
        }

        // Normalize weights of the kernel sothey sum to 1.0, to avoid having darker or brighter image
        for weight in kernel.iter_mut() {
            *weight /= total_weights;
        }

        Ok(Self {
            kernel,
            kernel_size,
        })
    }
}

impl Filtering for GaussianBlur {
    fn filter(&self, input_img: RgbaImage) -> Result<RgbaImage, Error> {
        let (width, height) = input_img.dimensions();

        let half_kernel_size = self.kernel_size / 2; // use to center kernel on pixel

        // apply kernel horizontaly using parallelization to compute each row independently
        let horizontal_pass_data: Vec<u8> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row_data: Vec<u8> = Vec::new();

                for x in 0..width {
                    for channel_index in 0..4 {
                        let mut gaussian_sum: f64 = 0.0;

                        for kx in 0..self.kernel_size {
                            // calculate coordinates of current neighbour pixel (shifted by half kernel size to center align Kernel with current pixel calculated (x,y))
                            let neighbour_x =
                                (x as i128) + (kx as i128) - (half_kernel_size as i128);

                            if neighbour_x >= 0 && neighbour_x < width as i128 {
                                let weight = self.kernel[kx];
                                gaussian_sum += (input_img
                                    .get_pixel_checked(neighbour_x as u32, y as u32)
                                    .unwrap()
                                    .channels()[channel_index]
                                    as f64)
                                    * weight;
                            }
                        }

                        //blurred_pixel[channel_index] = gaussian_sum as u8;
                        row_data.push(gaussian_sum as u8);
                    }
                }

                row_data
            })
            .flatten()
            .collect();

        let temp_image = RgbaImage::from_vec(width, height, horizontal_pass_data).unwrap(); // SAFETY as buffer is build from image width and weight

        // apply kernel verticaly using parallelization to compute each column independently
        let vertical_pass_data: Vec<Vec<u8>> = (0..width)
            .into_par_iter()
            .map(|x| {
                let mut column_data: Vec<u8> = Vec::new();

                for y in 0..height {
                    for channel_index in 0..4 {
                        let mut gaussian_sum: f64 = 0.0;

                        for ky in 0..self.kernel_size {
                            // calculate coordinates of current neighbour pixel (shifted by half kernel size to center align Kernel with current pixel calculated (x,y))
                            let neighbour_y =
                                (y as i128) + (ky as i128) - (half_kernel_size as i128);

                            if neighbour_y >= 0 && neighbour_y < height as i128 {
                                let weight = self.kernel[ky];
                                gaussian_sum += (temp_image
                                    .get_pixel_checked(x as u32, neighbour_y as u32)
                                    .unwrap()
                                    .channels()[channel_index]
                                    as f64)
                                    * weight;
                            }
                        }

                        column_data.push(gaussian_sum as u8);
                    }
                }

                column_data
            })
            .collect();

        // vertical buffer is a column major matrix, so we must rewrite data in resulting image properly
        let mut blurred_img = RgbaImage::new(width, height);
        for (x, column) in vertical_pass_data.iter().enumerate() {
            for y in 0..height {
                // get channel value for current pixel, as data have been flatten in a column vector
                let red = column.get((y * 4) as usize).ok_or(Error::UnfoundChannel)?;
                let green = column
                    .get((y * 4 + 1) as usize)
                    .ok_or(Error::UnfoundChannel)?;
                let blue = column
                    .get((y * 4 + 2) as usize)
                    .ok_or(Error::UnfoundChannel)?;
                let alpha = column
                    .get((y * 4 + 3) as usize)
                    .ok_or(Error::UnfoundChannel)?;

                let blurred_pixel = Rgba([*red, *green, *blue, *alpha]);
                blurred_img[(x as u32, y)] = blurred_pixel;
            }
        }

        Ok(blurred_img)
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
