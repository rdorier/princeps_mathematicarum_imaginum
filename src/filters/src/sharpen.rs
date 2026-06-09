use image::{Pixel, Rgba, RgbaImage};

use crate::{Error, Filtering};

pub struct Sharpen {
    kernel: Vec<Vec<f64>>,
}

impl Sharpen {
    pub fn new() -> Self {
        Self {
            kernel: vec![vec![0., -1., 0.], vec![-1., 5., -1.], vec![0., -1., 0.]],
        }
    }
}

impl Filtering for Sharpen {
    fn filter(&self, img: RgbaImage) -> Result<RgbaImage, Error> {
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

        Ok(sharpen_img)
    }
}
