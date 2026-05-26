# Princeps Mathematicarum Imaginum: scientia et ars in imagine coniunctae.

The Mathematic Prince of Image : science and art gathered. A little project aiming at manipulating images.  
The project name is a reference to Carl Friedrich Gauss, considered as the Prince of Mathematics.

## Command-Line Interface usage

The tool works as a CLI. You must pass three arguments : the first being the path to the input image you want to treat, the second one, the path to the resulting image, and the last one the operation to perform.  

The following argument is the operation type to perform :
- "-inverse" to perform a color inversion
- "-filter" to apply one of the available filters

## Available operations

### Inverse

Invert each pixel of the image, meaning that every white pixel became a black one, etc. Substantially, it substracts every channel value (ranged from 0 to 255, inclusive) of the pixel to 255. Use `-inverse` command to inverse an input image.  

![Test image as moutains landscape](doc/images/mountain-8487679_1920.jpg)
Example of an input image  
![Resulting inverted image](doc/images/inverse.png)
Resulting inverted image

## Available filters

### Edges Detection using Sobel Filter

Compute intensity gradient of the input image to detect edges. Use `-filter sobel` command to apply it to input image.

![Test image as moutains landscape](doc/images/mountain-8487679_1920.jpg)
Example of an input image  
![Resulting image with sobel filter](doc/images/sobel_filter.png)
Resulting edge detection using Sobel Filter

### Gaussian Blur

This algorithm blurs a given image using the Gaussian function. It takes a sigma value to define the size of the kernel used to blur every pixel of the given image. The Gaussian function is then used to fill the kernel with the neighbours weights. Use `-filter gaussian_blur` command to apply it to input image.

![Test image as moutains landscape](doc/images/mountain-8487679_1920.jpg)
Example of an input image  
![Resulting image with gaussian blur applied with 3.0 as sigma value](doc/images/gaussian_blur_sigma3.png)
Resulting blurred image with a sigma value of 3.0
![Resulting image with gaussian blur applied with 10.5 as sigma value](doc/images/gaussian_blur_sigma10.5.png)
Resulting blurred image with a sigma value of 10.5

## Goals :

1. Done : Basic pixels manipulation with `image` crate(read an image, and apply a simple transformation like color inversion)
2. Done : Implement a convolution filter (like Sobel one for edges detection)
3. WIP : Implement several filters (GaussianBlur, EdgeDetection, Sharpen) using POO
4. TODO : use `rayon` for parallel treatments and have quicker filters
5. WIP : create a reusable Rust crate with a simple CLI
6. TODO : add unit tests
