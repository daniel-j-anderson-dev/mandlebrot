use crate::{image_adapter::colors_to_rgbimage, terminal_input::get_parsed_input, *};
use num::Complex;
use std::time::Instant;

#[test]
pub fn default() -> Result<(), Box<dyn std::error::Error>> {
    let grand_start = Instant::now();

    const IMAGE_WIDTH: usize = 1000;
    const IMAGE_HEIGHT: usize = 1000;
    const SCALE_FACTOR: f64 = 1.0;
    const ORIGIN: Complex<f64> = Complex::new(0.0, 0.0);
    const ITERATION_MAX: usize = 500;

    let start = Instant::now();
    let color_data = calculate_color_data(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        SCALE_FACTOR,
        ORIGIN,
        ITERATION_MAX,
    );
    let color_delta = Instant::now() - start;

    let start = Instant::now();
    let output = colors_to_rgbimage(&color_data, IMAGE_WIDTH, IMAGE_HEIGHT);
    let image_delta = Instant::now() - start;

    let start = Instant::now();
    output.save(format!(
        "mandelbrot_{}x{}_{}_iter.png",
        IMAGE_WIDTH, IMAGE_HEIGHT, ITERATION_MAX
    ))?;
    let end = Instant::now();

    let save_delta = end - start;
    let grand_delta = end - grand_start;

    println!(
        r"
Resolution: {}x{}
Number of iterations: {}
Grand total: {:?}
Calculating pixel colors: {:?}
Copying raw pixels into image: {:?}
Saving: {:>?}",
        IMAGE_WIDTH, IMAGE_HEIGHT, ITERATION_MAX, grand_delta, color_delta, image_delta, save_delta,
    );

    Ok(())
}

#[test]
pub fn terminal() -> Result<(), Box<dyn std::error::Error>> {
    let image_width = get_parsed_input("Enter image width: ")?;
    let image_height = get_parsed_input("Enter image height: ")?;
    let scale_factor = get_parsed_input("Enter scale factor: ")?;
    let origin = get_parsed_input("Enter a complex number to be the origin of the image (eg. 1 + 2i): ")?;
    let iteration_max = get_parsed_input("Enter max number of iterations: ")?;
    
    let grand_start = Instant::now();
    let start = grand_start;
    let color_data = calculate_color_data(
        image_width,
        image_height,
        scale_factor,
        origin,
        iteration_max,
    );
    let color_delta = Instant::now() - start;

    let start = Instant::now();
    let output = colors_to_rgbimage(&color_data, image_width, image_height);
    let image_delta = Instant::now() - start;
    
    let start = Instant::now();
    output.save(format!(
        "mandelbrot_{}x{}_{}_iter.png",
        image_width, image_height, iteration_max
    ))?;
    let end = Instant::now();
    let save_delta = end - start;

    let grand_delta = end - grand_start;

    println!(
        r"
Resolution: {}x{}
Number of iterations: {}
Grand total: {:?}
Calculating pixel colors: {:?}
Copying raw pixels into image: {:?}
Saving: {:>?}",
        image_width, image_height, iteration_max, grand_delta, color_delta, image_delta, save_delta,
    );

    Ok(())
}
