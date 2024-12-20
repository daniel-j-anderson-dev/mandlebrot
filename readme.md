# Summary
A simple library that can generate raw pixel data of a portion of the mandelbrot set.

This library also supplys simple command line tools that can be helpfull for getting started quickly.
Right now only grayscale is supported, but supprot for color profiles is planned.

Can create: 
- <a src="https://docs.rs/image/latest/image/type.RgbImage.html">`image::color::RgbImage`</a>
- <a src="https://docs.rs/epaint/latest/epaint/image/struct.ColorImage.html">`epaint::image::ColorImage`</a>

# Quickstart Usage
after cloning source
use <b>`cargo test terminal`</b> to specify the image resolution, complex plane origin, viewport scale, and iteration cut off

# High Resolution Example
Image saved with <a src="https://docs.rs/image/0.24.7/image/">image</a> crate
<img src="mandelbrot_10000x10000_500_iter.png">
