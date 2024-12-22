use crate::{image_adapter::colors_to_rgbimage, *};
use num::Complex;
use std::time::Instant;

#[test]
pub fn default() -> Result<(), Box<dyn std::error::Error>> {
    let grand_start = Instant::now();

    const IMAGE_WIDTH: usize = 1000;
    const IMAGE_HEIGHT: usize = 1000;
    const TOP_LEFT: Complex<f64> = Complex::new(-2.0, 1.2);
    const BOTTOM_RIGHT: Complex<f64> = Complex::new(0.5, -1.2);
    const ITERATION_MAX: usize = 500;

    let start = Instant::now();
    let color_data = calculate_color_data(
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        TOP_LEFT,
        BOTTOM_RIGHT,
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

    let top_left = origin + Complex::new(-2.0, 1.2).scale(scale_factor);
    let bottom_right = origin + Complex::new(0.5, -1.2).scale(scale_factor);
    
    let grand_start = Instant::now();
    let start = grand_start;
    let color_data = calculate_color_data(
        image_width,
        image_height,
        top_left,
        bottom_right,
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

/// Prompts the user to enter a number.
///
/// # Parameters
/// - `prompt`: A string slice that will be printed before user input is read.
///
/// # Type Parameters
///  - `T`: A type that can be parsed from a string with a printable error case
///
/// # Returns
/// - `Ok(parsed_input)`: When the user inputs a valid instance of `T`
/// - `Err(io_error)`: When there is an io error from `get_line`
pub fn get_parsed_input<T>(prompt: &str) -> Result<T, std::io::Error>
where
    T: core::str::FromStr,                // Needs to be parsable
    T::Err: core::error::Error, // Need to be able to print error if parse fails
{
    // keep trying until the user gets enters a valid instance of T
    loop {
        match get_input(prompt)?.parse() {
            Ok(number_input) => return Ok(number_input),
            Err(parse_error) => eprintln!("\nInvalid input: {}\n", parse_error),
        }
    }
}

/// Reads a line of input from stdin
fn get_input(prompt: &str) -> Result<String, std::io::Error> {
    use std::io::Write;

    {
        let mut stdout = std::io::stdout();
        stdout.write(prompt.as_bytes())?;
        stdout.flush()?;
    }

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input.truncate(input.trim_end().len());

    Ok(input)
}
