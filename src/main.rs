use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use filters::{Filtering, GaussianBlur, SobelFilter};
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
    GammaCorrection{
        gamma: f64
    },
    Filter {
        //specify the type of filter to apply
        filter_type: String,
        // an additional parameter, depending on filter type
        filter_parameter: Option<f64>,
    }
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
        Commands::GammaCorrection{gamma} => {
            let corrected_img = gamma_correction(&img, gamma).with_context(|| "Failed to correct gamma")?;
            Ok(corrected_img)
        },
        Commands::Filter { filter_type, filter_parameter } => {
            match filter_type.as_str() {
                "sobel" => {
                    let sobel_filter = SobelFilter;
                    Ok(sobel_filter.filter(img))
                }
                "gaussian_blur" => {
                    let sigma = filter_parameter.unwrap_or(3.0);
                    let gaussian_blur_filter = GaussianBlur::try_new(sigma)
                        .with_context(|| "Failed to initialize Gaussian Blur kernel")?;
                    Ok(gaussian_blur_filter.filter(img))
                }
                _ => {
                    Err(Error::UnknownFilter(filter_type))
                }
            }
        }
    }?;

    // save resulting image
    resulting_img.save(output_file.clone())
        .with_context(|| format!("Failed to save output file {output_file}"))?;
    println!("Resulting image saved at location : {output_file}");


    Ok(())
}
