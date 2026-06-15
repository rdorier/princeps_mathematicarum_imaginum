use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use filters::{Filtering, GaussianBlur, Sharpen, SharpenAlgorithm, SobelFilter};
use image_processing::{gamma_correction, inverse, read_image_from_path};

mod errors;
use errors::*;

#[derive(Parser)]
struct Cli {
    // path of the file to process
    input_file_path: String,
    // The path where to store resulting image
    output_file_path: String,
    // the operation to perform
    #[command(subcommand)]
    operation: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Inverse,
    GammaCorrection {
        gamma: f64,
    },
    Filter {
        /// Specify the type of filter to apply.
        /// This is a positional required argument
        filter_type: String,

        /// Parameter used to specify sigma value to give to some filters
        /// This optional parameter is an explicit flag
        #[arg(long = "sigma")]
        sigma: Option<f64>,

        /// Parameter used to specify the algorithm name to use for some filters
        /// This optional parameter is an explicit flag
        #[arg(long = "algorithm")]
        algorithm: Option<String>,
    },
}

fn main() -> Result<(), Error> {
    // parse arguments pass to the tool using CLI
    let args = Cli::parse();

    // get path of image to treat
    let input_file = args.input_file_path;
    // get path to the resulting image to produce
    let output_file = args.output_file_path;

    let img = read_image_from_path(&input_file)
        .with_context(|| format!("Could not read file `{}`", input_file))?;

    println!("Image dimension : {:?}", img.dimensions());

    let resulting_img = match args.operation {
        Commands::Inverse => Ok(inverse(img)),
        Commands::GammaCorrection { gamma } => {
            let corrected_img =
                gamma_correction(&img, gamma).with_context(|| "Failed to correct gamma")?;
            Ok(corrected_img)
        }
        Commands::Filter {
            filter_type,
            sigma,
            algorithm,
        } => match filter_type.as_str() {
            "sobel" => {
                let sobel_filter = SobelFilter;
                sobel_filter.filter(img).map_err(Error::FailedFilter)
            }
            "gaussian_blur" => {
                let sigma = sigma.unwrap_or(3.0);
                let gaussian_blur_filter = GaussianBlur::try_new(sigma)
                    .with_context(|| "Failed to initialize Gaussian Blur kernel")?;
                gaussian_blur_filter
                    .filter(img)
                    .map_err(Error::FailedFilter)
            }
            "sharpen" => {
                let algorithm_name = algorithm.unwrap_or(String::from("default"));
                let algorithm = match algorithm_name.as_str() {
                    "laplacian" => SharpenAlgorithm::Laplacian,
                    "usm" => SharpenAlgorithm::UnsharpMasking,
                    _ => SharpenAlgorithm::DefaultKernel,
                };
                let sharpen_filter = Sharpen::new(Some(algorithm));
                sharpen_filter.filter(img).map_err(Error::FailedFilter)
            }
            _ => Err(Error::UnknownFilter(filter_type)),
        },
    }?;

    // save resulting image
    resulting_img
        .save(output_file.clone())
        .with_context(|| format!("Failed to save output file {output_file}"))?;
    println!("Resulting image saved at location : {output_file}");

    Ok(())
}
