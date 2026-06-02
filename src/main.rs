use anyhow::{Context, Result};
use clap::Parser;
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
    operation: String,
    //specify the type of filter to apply. Only mandatory/usefull when operation = filter
    filter_type:Option<String>,
    // an additional parameter, depending on filter type
    filter_parameter: Option<f64>,
}


fn main() -> Result<(), Error> {
    // parse arguments pass to the tool using CLI
    let args = Cli::parse();

    // get path of image to treat
    let input_file = args.input_file_path;
    // get path to the resulting image to produce
    let output_file = args.output_file_path;

    let operation = args.operation;

    let img = read_image_from_path(&input_file)
        .with_context(|| format!("Could not read file `{}`", input_file))?;

    println!("Image dimension : {:?}", img.dimensions());

    let resulting_img = match operation.as_str() {
        "inverse" => Ok(inverse(img)),
        "gamma_correction" => Ok(gamma_correction(&img, 0.8)),
        "filter" => {
            let filter_type = args.filter_type.ok_or(Error::NotEnoughArguments)?;
            match filter_type.as_str() {
                "sobel" => {
                    let sobel_filter = SobelFilter;
                    Ok(sobel_filter.filter(img))
                }
                "gaussian_blur" => {
                    let sigma = args.filter_parameter.unwrap_or(3.0);
                    let gaussian_blur_filter = GaussianBlur::try_new(sigma)
                        .with_context(|| "Failed to initialize Gaussian Blur kernel")?;
                    Ok(gaussian_blur_filter.filter(img))
                }
                _ => {
                    Err(Error::UnknownFilter(filter_type))
                }
            }
        }
        _ => {
            Err(Error::UnknownOperation(operation.clone()))
        }
    }?;

    // save resulting image
    resulting_img.save(output_file.clone())
        .with_context(|| format!("Failed to save output file {output_file}"))?;
    println!("Resulting image saved at location : {output_file}");


    Ok(())
}
