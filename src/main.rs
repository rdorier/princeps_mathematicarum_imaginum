use anyhow::{Context, Result};
use filters::{Filtering, GaussianBlur, SobelFilter};
use image_processing::{inverse, read_image_from_path};
use std::env;

mod errors;
use errors::*;

#[derive(PartialEq)]
enum CLICommandValidity {
    ValidCommand,
    InvalidCommand,
}

fn main() -> Result<(), Error> {
    // check if CLI command enter by user is valid
    let mut command_validity = CLICommandValidity::ValidCommand;

    // collect arguments pass to the tool using CLI
    let args: Vec<String> = env::args().collect();

    // check parameters number
    if args.len() < 4 {
        return Err(Error::NotEnoughArguments);
    }

    // get path of image to treat
    let input_file = &args[1];
    // get path to the resulting image to produce
    let output_file = &args[2];

    let operation = &args[3];

    let mut img = read_image_from_path(input_file)
        .with_context(|| format!("Could not read file `{}`", input_file))?;

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
                    }
                    "gaussian_blur" => {
                        let sigma = if args.len() >= 6 {
                            args[5].parse().with_context(|| {
                                format!("Sigma argument value {} is not a number", args[5])
                            })?
                        } else {
                            3.0
                        };
                        let gaussian_blur_filter = GaussianBlur::try_new(sigma);
                        gaussian_blur_filter.filter(img)
                    }
                    _ => {
                        // TODO : return custom error and display message with context instead of enum
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
            .with_context(|| format!("Failed to save output file {output_file}"))?;
        println!("Resulting image saved at location : {output_file}");
    } else {
        // exit if invalid operation requested
        println!("Invalid operation requested ! Nothing to do here !")
    }

    Ok(())
}
