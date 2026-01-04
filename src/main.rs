use std::env;
use image::ImageReader;
use image::Pixel;

fn main() -> Result<(), image::ImageError> {
    // collect arguments pass to the tool using CLI
    let args: Vec<String> = env::args().collect();

    //
    let input_file = &args[1];
    let output_file = &args[2];

    let img = ImageReader::open(input_file)?.decode()?;
    let mut img = img.to_rgba8();

    println!("Image dimension : {:?}", img.dimensions());

    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    img.save(output_file)?;
    Ok(())
}
