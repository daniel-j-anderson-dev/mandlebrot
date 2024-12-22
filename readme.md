# Summary
Generates raw pixel data of a portion of the [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set).

This library also supplies simple command line tools that can be helpful for getting started quickly.
Right now only gray-scale is supported, but support for color profiles is planned.

## Features
- `image`: allows for creating [`image::color::RgbImage`](https://docs.rs/image/latest/image/type.RgbImage.html)s
- `epaint`: allows for creating [`epaint::image::ColorImage`]("https://docs.rs/epaint/latest/epaint/image/struct.ColorImage.html")s

## Quickstart Usage
1. clone this repository `git clone https://github.com/daniel-j-anderson-dev/mandlebrot.git`
2. navigate to repo directory `cd mandelbrot`
3. run the test from the terminal `cargo test terminal --release --features image`
4. the generated image will be in the repository root

## High Resolution Example
10000x10000
500 iterations
<img src="mandelbrot_10000x10000_500_iter.png">
