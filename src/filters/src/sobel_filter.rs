use ndarray::{Array2, array};
use crate::Filtering;
use image::RgbaImage;
use image_processing::{gray_array_to_rgba_conversion, rgba_to_gray_array_conversion};

pub struct SobelFilter;

impl Filtering for SobelFilter {
    fn filter(&self, img: RgbaImage) -> RgbaImage {
        // Detect edges using Sobel algorithm.

        // sobel kernel to use to find intensity peaks in image : the intensity peaks represent edges
        // this kernel highlight vertical edges
        let sobel_x_matrix = array![[-1., 0., 1.], [-2., 0., 2.], [-1., 0., 1.],];
        // this kernel highlight horizontal edges
        let sobel_y_matrix = array![[-1., -2., -1.], [0., 0., 0.], [1., 2., 1.],];

        // convert image as intensity array (grayscale value)
        let img_as_gray_array = rgba_to_gray_array_conversion(&img);
        let (height, width) = img_as_gray_array.dim();

        let mut edges_array = Array2::<f32>::zeros((height, width));

        // convolve intensity array with kernel
        for i in 1..((height - 1) as i32) {
            for j in 1..((width - 1) as i32) {
                let mut sumx = 0.0;
                let mut sumy = 0.0;

                for p in -1i32..=1 {
                    for q in -1i32..=1 {
                        let y: usize = (i + p).try_into().unwrap();
                        let x: usize = (j + q).try_into().unwrap();

                        let kernel_x: usize = (p + 1).try_into().unwrap();
                        let kernel_y: usize = (q + 1).try_into().unwrap();

                        sumx += img_as_gray_array[[y, x]] * sobel_x_matrix[[kernel_x, kernel_y]];
                        sumy += img_as_gray_array[[y, x]] * sobel_y_matrix[[kernel_x, kernel_y]];
                    }
                }
                edges_array[[i as usize, j as usize]] = ((sumx * sumx) + (sumy * sumy)).sqrt();
            }
        }

        // convert array into a grayscale image
        gray_array_to_rgba_conversion(&edges_array)
    }
}