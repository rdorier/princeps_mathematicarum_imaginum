# Princeps Mathematicarum Imaginum: scientia et ars in imagine coniunctae.

The Mathematic Prince of Image : science and art gathered. A little project aiming at manipulating images.  
The project name is a reference to Carl Friedrich Gauss, considered as the Prince of Mathematics.  

## Command-Line Interface usage

The tool works as a CLI. You must pass two arguments : the first being the path to the input image you want to treat, and the second one, the path to the resulting image.

## Edges Detection using Sobel Filter

DOC to do

![Test image as moutains landscape](doc/images/mountain-8487679_1920.jpg)
![Resulting image with sobel filter](doc/images/sobel_filter.png)

## Goals :
1. Done : Basic pixels manipulation with `image` create(read an image, and apply a simple transformation like color inversion)
2. TODO : Implement a convolution filter (like Sobel one for edges detection)
3. TODO : Implement several filters (GaussianBlur, EdgeDetection, Sharpen) using POO
4. TODO : use `rayon` for parallel treatments and have quicker filters
5. TODO : create a reusable Rust crate with a simple CLI
6. TODO : add unit tests
