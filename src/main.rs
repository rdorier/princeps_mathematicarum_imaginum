use std::env;
use image::{RgbaImage, ImageReader, Pixel, Rgba};
use anyhow::{Context, Result};
use ndarray::{array, Array2};


fn rgba_to_gray_array_conversion(img: &RgbaImage) -> Array2<f32> {
    // transform image as grayscale using coef from UIT-R BT.601-7 norm to mimic human perception
    let green_sensibility_coef = 0.299;
    let red_sensibility_coef = 0.587;
    let blue_sensibility_coef = 0.114;

    let (width, height) = img.dimensions();
    let mut gray_img = Array2::<f32>::zeros((height as usize, width as usize));

    for (x, y, pixel) in img.enumerate_pixels() {
        let [r, g, b, _a] = pixel.0;
        let gray_value = green_sensibility_coef * (r as f32)
              + red_sensibility_coef * (g as f32)
              + blue_sensibility_coef * (b as f32);
        gray_img[[y as usize, x as usize]] = gray_value;
    }

    gray_img
}

fn gray_array_to_rgba(mag: &Array2<f32>) -> RgbaImage {
    let (h, w) = mag.dim();
    let mut img = RgbaImage::new(w as u32, h as u32);

    for y in 0..h {
        for x in 0..w {
            let v = mag[[y, x]].clamp(0.0, 255.0) as u8;
            img.put_pixel(x as u32, y as u32, Rgba([v, v, v, 255]));
        }
    }

    img
}



fn inverse(mut img : RgbaImage ) -> RgbaImage  {
    // Invert pixels value of the input image.
    for pixel in img.pixels_mut(){
        pixel.invert();
    }

    img
}

fn sobel_filter(img : RgbaImage ) -> RgbaImage  {
    // Detect edges using Sobel algorithm.
    println!("FILTER !");

    //let mut test = img.clone();

    let sobel_x_matrix = array![
        [-1., 0., 1.],
        [-2., 0., 2.],
        [-1., 0., 1.],
    ];

    let sobel_y_matrix = array![
        [-1., -2., -1.],
        [ 0.,  0.,  0.],
        [ 1.,  2.,  1.],
    ];
    println!("{}", sobel_y_matrix[[0,1]]);

    let test = rgba_to_gray_array_conversion(&img);
    let test = gray_array_to_rgba(&test);


    println!("{}", sobel_x_matrix);
    test
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

    // TODO exit if invalid operation requested

    // save resulting image
    img.save(output_file)
        .with_context(|| format!("Failed to save output file '{}'", output_file))?;
    println!("Resulting image saved at location : {output_file}");

    Ok(())
}
