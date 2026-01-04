use std::env;
use image::ImageReader;
use image::Pixel;

fn main() -> Result<(), image::ImageError> {
    // collect arguments pass to the tool using CLI
    let args: Vec<String> = env::args().collect();

    // get path of image to treat
    let input_file = &args[1];
    // get path to the resulting image to produce
    let output_file = &args[2];

    // open input image and convert it to RGBA
    let img = ImageReader::open(input_file)?.decode()?;
    let mut img = img.to_rgba8();

    println!("Image dimension : {:?}", img.dimensions());

    // invert pixels value of the input image
    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    // save resulting image
    img.save(output_file)?;
    Ok(())
}
