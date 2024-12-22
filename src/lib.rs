use num::Complex;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

#[cfg(feature = "epaint")]
pub mod epaint_adapter;

#[cfg(feature = "image")]
pub mod image_adapter;

#[cfg(test)]
pub mod test;

/// 8 bit r, g, b
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Color(u8, u8, u8);

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
    pixel_x: usize,
    pixel_y: usize,
    image_width: usize,
    image_height: usize,
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
) -> Complex<f64> {
    // calculate the region of the complex plane to map the pixel onto
    let complex_plane_width = bottom_right.re - top_left.re;
    let complex_plane_height = bottom_right.im - top_left.im;

    // determine the pixels position as a percentage of the image resolution
    let horizontal_ratio = pixel_x as f64 / image_width as f64;
    let vertical_ratio = pixel_y as f64 / image_height as f64;

    // scale the complex region dimensions by the percentage to get the relative complex position
    let offset = Complex::new(
        complex_plane_width * horizontal_ratio,
        complex_plane_height * vertical_ratio,
    );

    top_left + offset
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
/// - `iteration_max`: The amount of iterations to cutoff and consider a point part of the mandelbrot set
///
/// # Returns
/// - `Vec<Color>`: An
pub fn calculate_color_data(
    image_width: usize,
    image_height: usize,
    top_left: Complex<f64>,
    bottom_right: Complex<f64>,
    iteration_max: usize,
) -> Vec<Color> {
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
