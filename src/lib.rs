pub mod terminal_input;

use num::complex::Complex64;
use rayon::iter::{
    ParallelIterator,
    IntoParallelIterator
};

/// r, g, b color
#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);
impl Into<epaint::Color32> for Color {
    fn into(self) -> epaint::Color32 {
        epaint::Color32::from_rgb(self.0, self.1, self.2)
    }
}
impl Into<image::Rgb<u8>> for Color {
    fn into(self) -> image::Rgb<u8> {
        image::Rgb([self.0, self.1, self.2])
    }
}

/// position and color
#[derive(Clone, Copy)]
pub struct Pixel {
    pub x: usize,
    pub y: usize,
    pub color: Color,
}

pub fn complex_to_grayscale(c: Complex64, iteration_max: usize) -> Color {
    match escape_time(c, iteration_max) {
        Some(itteration_count) => {
            Color(itteration_count as u8, itteration_count as u8, itteration_count as u8)
        },
        None => {
            Color(0, 0, 0)
        },
    }
}

/// map any pixel position to a corrosponding complex number given the image resolution and a rectangular area on the complex plane
/// 
/// # Parameters
/// - `x`, `y`: pixel position
/// - `image_width`, `image_height`: image resolution  
/// - `top_left`: top left complex point of a rectangle
/// - `bottom_right`: bottom right complex point of a rectangle
/// 
/// # Returns
/// - <a src="https://docs.rs/num/latest/num/struct.Complex.html">`num::Complex<f64>`</a>: A unique complex number calculated from params
pub fn pixel_to_complex(
    // pixel coords
    x: usize,
    y: usize,

    // image resolution
    image_width: usize,
    image_height: usize,

    // rectangular area on complex plane
    top_left: Complex64,
    bottom_right: Complex64,
) -> Complex64
{
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
    
    Complex64::new(real, imaginary)
}

/// Calculate `zᵢₜₑᵣₐₜᵢₒₙₛ_ₘₐₓ` using: `zₙ₊₁ = zₙ² + c`.
/// 
/// z₀ = 0+0i 
/// 
/// Escape condition: ||zₙ² + c|| > 2
/// 
/// # Params
/// - `c`: A complex number to be checked for membership in the Mandelbrot set.
/// - `iteration_max`
/// 
/// # Returns
/// - `Some(n)`: When `zₙ` escapes within the specified maximum iterations.
/// - `None`: If `zₙ` does not escape within the given maximum iterations and is considered part of the Mandelbrot set.
pub fn escape_time(c: Complex64, iteration_max: usize) -> Option<usize> {
    let mut z = Complex64::new(0.0, 0.0);
    for n in 0..iteration_max {
        z = (z * z) + c;
        if z.norm_sqr() > 4.0 {
            return Some(n); // not in mandlebrot set
        }
    }
    None // In mandlebrot set
}

/// Caluculate color for each pixel using parallel iterators
pub fn calculate_pixel_data(
    image_width: usize,
    image_height: usize,
    scale_factor: f64,
    origin: Complex64,
    iteration_max: usize,
) -> Vec<Pixel> {
    let top_left = origin + Complex64::new(-2.0, 1.2).scale(scale_factor);
    let bottom_right = origin + Complex64::new(0.5, -1.2).scale(scale_factor);
    // iterate over every pixel position
    (0..image_height).into_par_iter().flat_map(|pixel_y| {
        (0..image_width).into_par_iter().map(move |pixel_x| {
            // turn pixel position into a specific complex number
            let c = pixel_to_complex(
                pixel_x, pixel_y,
                image_width, image_height,
                top_left, bottom_right
            );
            
            // calculate color of the specific complex number
            let color = complex_to_grayscale(c, iteration_max);
            
            // map each pixel position pair to a specific `Pixel`
            Pixel { x: pixel_x, y: pixel_y, color }
        }
    )}).collect()
}
