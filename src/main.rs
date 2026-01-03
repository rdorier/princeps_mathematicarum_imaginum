use image::ImageReader;
use image::Pixel;

fn main() -> Result<(), image::ImageError> {
    let img = ImageReader::open("C:/Users/romai/Pictures/Screenshots/Capture d'écran 2025-09-29 222849.png")?.decode()?;
    //img.save("D:/DATA/Romain/Documents/Dev/princeps_mathematicarum_imaginum/src/test.jpg")?;
    let mut img = img.to_rgba8();

    println!("Image dimension : {:?}", img.dimensions());

    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    img.save("D:/DATA/Romain/Documents/Dev/princeps_mathematicarum_imaginum/src/test.png")?;
    Ok(())
}
