use anyhow::{Context, Result, anyhow};
use filters::{Filtering, GaussianBlur, SobelFilter};
use image::{ImageReader, RgbaImage};
use image_processing::inverse;
use ndarray::{Array2, array};
use std::{env};



#[derive(PartialEq)]
enum CLICommandValidity {
    ValidCommand,
    InvalidCommand,
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
                    "sobel" => {
                        let sobel_filter = SobelFilter;
                        sobel_filter.filter(img)
                    },
                    "gaussian_blur" => {
                        let sigma = if args.len() >= 6 {args[5].parse()?} else {3.0};
                        let gaussian_blur_filter = GaussianBlur::try_new(sigma);
                        gaussian_blur_filter.filter(img)
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
