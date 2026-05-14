use anyhow::{Context, Result, anyhow};
use image::{ImageReader, Pixel, Rgba, RgbaImage};
use ndarray::{Array2, array};
use std::{env, f64::consts::PI};

mod image_transforms;
pub use crate::image_transforms::*;

#[derive(PartialEq)]
enum CLICommandValidity {
    ValidCommand,
    InvalidCommand,
}

fn sobel_filter(img: RgbaImage) -> RgbaImage {
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

/// Apply gaussian blur to given image.
///
/// # Arguments
/// * `img` - The image to blur.
/// * `sigma` - The standard deviation controlling the blur spread. Must be > 0.
fn gaussian_blur(img: RgbaImage, sigma: f64) -> RgbaImage {
    if sigma < 0.0 {
        panic!("Sigma value must be greater than 0"); // TODO : better error management
    }

    let mut kernel_size = ((6.0 * sigma).ceil() as usize).max(3);
    if kernel_size.is_multiple_of(2) {
        kernel_size += 1
    };

    // TODO : use separability property of the gaussian function to apply two 1-dimensional (one on rows, the other one on columns)

    // row major matrix representing gaussian kernel to apply to pixels : first index represents the row, second one the columns
    let mut gaussian_kernel = vec![vec![0.0; kernel_size]; kernel_size];

    for y in 0..kernel_size {
        for x in 0..kernel_size {
            // 2D Gaussian function: G(x, y) = (1 / (2 * pi * sigma^2)) * exp(-(x^2 + y^2) / (2 * sigma^2))
            gaussian_kernel[y][x] = (1.0 / (2.0 * PI * sigma * sigma))
                * (-((x * x + y * y) as f64) / (2.0 * sigma * sigma)).exp();
        }
    }

    let (width, height) = img.dimensions();

    let mut blurred_image = RgbaImage::new(width, height);

    for y in 1..height {
        for x in 1..width {
            let half_kernel_size = kernel_size / 2; // use to center kernel on pixel

            let mut blurred_pixel: Rgba<u8> = Rgba([0; 4]);

            for channel_index in 0..4 {
                let mut gaussian_sum: f64 = 0.0;
                let mut accumulated_weight = 0.0;

                for ky in 0..kernel_size {
                    for kx in 0..kernel_size {
                        // calculate coordinates of current neighbour pixel (shifted by hald kernel size to center align Kernel with current pixel calculated (x,y))
                        let neighbour_x = (x as i128) + (kx as i128) - (half_kernel_size as i128);
                        let neighbour_y = (y as i128) + (ky as i128) - (half_kernel_size as i128);

                        if neighbour_x >= 0
                            && neighbour_x < width as i128
                            && neighbour_y >= 0
                            && neighbour_y < height as i128
                        {
                            let weight = gaussian_kernel[ky][kx];

                            gaussian_sum += (img
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

    // convert array into a grayscale image
    blurred_image
}

fn main() -> Result<()> {
    // check if CLI command enter by user is valid
    let mut command_validity = CLICommandValidity::ValidCommand;

    // collect arguments pass to the tool using CLI
    let args: Vec<String> = env::args().collect();

    // check parameters number
    if args.len() < 4 {
        return Err(anyhow!(
            "Not enough arguments given to the CLI. You must at least specify an input image, an output location and an operation to perform."
        ));
    }

    // get path of image to treat
    let input_file = &args[1];
    // get path to the resulting image to produce
    let output_file = &args[2];

    let operation = &args[3];

    // try to open input image
    let reader = ImageReader::open(input_file)
        .with_context(|| format!("Failed to open input file '{}'", input_file))?
        .with_guessed_format()
        .context("Could not guess image format")?;

    // decode input image and convert it to RGBA
    let img = reader.decode().context("Failed to decode image")?;
    let mut img = img.to_rgba8();

    println!("Image dimension : {:?}", img.dimensions());

    match operation.as_str() {
        "-inverse" => img = inverse(img),
        "-filter" => {
            if args.len() >= 5 {
                let filter_type = args[4].clone();
                img = match filter_type.as_str() {
                    "sobel" => sobel_filter(img),
                    "gaussian_blur" => {
                        let sigma = if args.len() >= 6 {args[5].parse()?} else {3.0};
                        gaussian_blur(img, sigma)
                    },
                    _ => {
                        println!("Unknown filter name : {filter_type}");
                        command_validity = CLICommandValidity::InvalidCommand;
                        img
                    }
                }
            }
        }
        _ => {
            println!("Unknown requested operation : {operation}");
            command_validity = CLICommandValidity::InvalidCommand
        }
    }

    if command_validity == CLICommandValidity::ValidCommand {
        // save resulting image
        img.save(output_file)
            .with_context(|| format!("Failed to save output file '{}'", output_file))?;
        println!("Resulting image saved at location : {output_file}");
    } else {
        // exit if invalid operation requested
        println!("Invalid operation requested ! Nothing to do here !")
    }

    // TODO : improve error and result handling
    Ok(())
}
