use anyhow::{Context, Result};
use filters::{Filtering, GaussianBlur, SobelFilter};
use image_processing::{inverse, read_image_from_path};
use std::env;

mod errors;
use errors::*;



fn main() -> Result<(), Error> {
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

    let img = read_image_from_path(input_file)
        .with_context(|| format!("Could not read file `{}`", input_file))?;

    println!("Image dimension : {:?}", img.dimensions());

    let resulting_img = match operation.as_str() {
        "-inverse" => Ok(inverse(img)),
        "-filter" => {
            if args.len() >= 5 {
                let filter_type = args[4].clone();
                match filter_type.as_str() {
                    "sobel" => {
                        let sobel_filter = SobelFilter;
                        Ok(sobel_filter.filter(img))
                    }
                    "gaussian_blur" => {
                        let sigma = if args.len() >= 6 {
                            args[5].parse().with_context(|| {
                                format!("Sigma argument value {} is not a number", args[5])
                            })?
                        } else {
                            3.0
                        };
                        let gaussian_blur_filter = GaussianBlur::try_new(sigma)
                            .with_context(|| "Failed to initialize Gaussian Blur kernel")?;
                        Ok(gaussian_blur_filter.filter(img))
                    }
                    _ => {
                        Err(Error::UnknownFilter(filter_type))
                    }
                }
            }
            else {
                Err(Error::NotEnoughArguments)
            }
        }
        _ => {
            Err(Error::UnknownOperation(operation.clone()))
        }
    }?;

    // save resulting image
    resulting_img.save(output_file)
        .with_context(|| format!("Failed to save output file {output_file}"))?;
    println!("Resulting image saved at location : {output_file}");


    Ok(())
}
