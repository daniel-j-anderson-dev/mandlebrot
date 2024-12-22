use num::Complex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub mod terminal_input;
#[cfg(test)]
pub mod test;

/// 8 bit r, g, b
#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);
impl From<Color> for epaint::Color32 {
    fn from(value: Color) -> Self {
        epaint::Color32::from_rgb(value.0, value.1, value.2)
    }
}
impl From<&Color> for epaint::Color32 {
    fn from(value: &Color) -> Self {
        epaint::Color32::from_rgb(value.0, value.1, value.2)
    }
}
impl From<Color> for image::Rgb<u8> {
    fn from(value: Color) -> Self {
        image::Rgb([value.0, value.1, value.2])
    }
}
impl From<&Color> for image::Rgb<u8> {
    fn from(value: &Color) -> Self {
        image::Rgb([value.0, value.1, value.2])
    }
}

/// For interfacing with `image` crate
pub fn colors_to_rgbimage(color_data: &[Color], width: usize, height: usize) -> image::RgbImage {
    image::ImageBuffer::from_par_fn(width as u32, height as u32, |x, y| {
        color_data[(y as usize * width) + x as usize].into()
    })
}

/// For interfacing with `epaint` crate
pub fn colors_to_colorimage(
    color_data: &[Color],
    width: usize,
    height: usize,
) -> epaint::ColorImage {
    let mut output = epaint::ColorImage::new([width, height], epaint::Color32::from_rgb(0, 0, 0));
    output.pixels = color_data
        .into_par_iter()
        .map(epaint::Color32::from)
        .collect();
    output
}

/// # Parameters
/// - `c`: A complex number to be colored depending on its `escape_time`
/// - `iteration_max`: The max number of iterations for `escape_time`
///
/// # Returns
/// - `Color`: raw rgb values.<br>
///  Black if not in mandelbrot set or if c escaped immediately.
///  Otherwise `escape_iter_count` is used for r, g, and b
pub fn escape_time_to_grayscale(escape_time: Option<usize>) -> Color {
    match escape_time {
        Some(escape_iter_count) => Color(
            escape_iter_count as u8,
            escape_iter_count as u8,
            escape_iter_count as u8,
        ),
        None => Color(0, 0, 0),
    }
}

/// map any pixel position to a corresponding complex number given the image resolution and a rectangular area on the complex plane
///
/// # Parameters
/// - `pixel_x`, `pixel_y`: pixel position. (top left pixel is `(0, 0)`)
/// - `image_width`, `image_height`: image resolution  
/// - `top_left`: top left complex point of a rectangle
/// - `bottom_right`: bottom right complex point of a rectangle
///
/// # Returns
/// - [Complex]: A unique complex number calculated from params
pub fn pixel_to_complex(
    x: usize,
    y: usize,
    image_width: usize,
    image_height: usize,
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) -> Complex<f64> {
    // determine complex bounds
    let left_bound = top_left.re;
    let top_bound = top_left.im;
    let bottom_bound = bottom_right.im;
    let right_bound = bottom_right.re;

    // scale each point (x, y)*(x/width, y/height)
    let rect_width = (right_bound - left_bound) * x as f64 / image_width as f64;
    let rect_height = (bottom_bound - top_bound) * y as f64 / image_height as f64;

    // scale complex bounds by their respective percentages then add to the initial bounds
    let real = left_bound + rect_width;
    let imaginary = top_bound + rect_height;

    Complex::new(real, imaginary)
}

/// Calculate `zᵢₜₑᵣₐₜᵢₒₙₛ_ₘₐₓ` using: `zₙ₊₁ = zₙ² + c`.
///
/// `z₀ = 0+0i`
///
/// Escape condition: `||zₙ² + c|| > 2`
///
/// # Params
/// - `c`: A complex number to be checked for membership in the Mandelbrot set.
/// - `iteration_max`
///
/// # Returns
/// - `Some(n)`: When `zₙ` escapes within the specified maximum iterations.
/// - `None`: If `zₙ` does not escape within the given maximum iterations and is considered part of the Mandelbrot set.
pub fn escape_time(c: Complex<f64>, iteration_max: usize) -> Option<usize> {
    let mut z = Complex::new(0.0, 0.0);
    for n in 0..iteration_max {
        z = (z * z) + c;
        if z.norm_sqr() > 4.0 {
            return Some(n); // not in mandelbrot set
        }
    }
    None // In mandelbrot set
}

/// Calculate color based on the `escape_time` of the each pixel using parallel iterators
///
/// # Parameters
/// - `image_width`, `image_height`: image resolution  
/// - `origin`: the origin of the viewing rectangular area on the complex plane
/// - `iteration_max`: The amount of iterations to cuttoff and consider a point part of the mandelbrot set
///
/// # Returns
/// - `Vec<Color>`: An
pub fn calculate_pixel_data(
    image_width: usize,
    image_height: usize,
    scale_factor: f64,
    origin: Complex<f64>,
    iteration_max: usize,
) -> Vec<Color> {
    let top_left = origin + Complex::new(-2.0, 1.2).scale(scale_factor);
    let bottom_right = origin + Complex::new(0.5, -1.2).scale(scale_factor);

    // iterate over every pixel position in parallel
    // mapping each pixel position to a specific `Color`
    let pixel_indexes = (0..image_height)
        .into_par_iter()
        .flat_map(|y| (0..image_width).into_par_iter().map(move |x| (y, x)));

    pixel_indexes
        .map(|(y, x)| {
            // turn pixel position into a specific complex number
            let c = pixel_to_complex(x, y, image_width, image_height, top_left, bottom_right);

            // calculate the mandelbrot equation the specified amount of iterations
            let escape_time = escape_time(c, iteration_max);

            // calculate color of the specific complex number
            escape_time_to_grayscale(escape_time)
        })
        .collect()
}
