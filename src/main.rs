use std::env;
use image::RgbaImage;
use image::ImageReader;
use image::Pixel;
use anyhow::{Context, Result};


fn inverse(mut img : RgbaImage ) -> RgbaImage  {
    println!("INVERSE !");

    // invert pixels value of the input image
    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    img
}

fn sobel_filter(mut img : RgbaImage ) -> RgbaImage  {
    println!("FILTER !");
    img
}


fn main() -> Result<()> {
    // collect arguments pass to the tool using CLI
    let args: Vec<String> = env::args().collect();

    // TODO : check parameters number

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
        "-filter" => img = sobel_filter(img),
        _ => println!("Unknown requested operation : {operation}"),
    }

    // save resulting image
    img.save(output_file)
        .with_context(|| format!("Failed to save output file '{}'", output_file))?;
    println!("Resulting image saved at location : {output_file}");

    Ok(())
}
