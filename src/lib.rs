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
pub struct Color([u8; 4]);
impl Color {
    pub const fn as_array(&self) -> [u8; 4] {
        self.0
    }
    pub const fn as_slice(&self) -> &[u8; 4] {
        &self.0
    }
    pub const fn as_slice_mut(&mut self) -> &mut [u8; 4] {
        &mut self.0
    }
    pub const fn red(&self) -> u8 {
        self.0[0]
    }
    pub const fn green(&self) -> u8 {
        self.0[1]
    }
    pub const fn blue(&self) -> u8 {
        self.0[2]
    }
    pub const fn alpha(&self) -> u8 {
        self.0[3]
    }
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
        Some(escape_iter_count) => Color([
            escape_iter_count as u8,
            escape_iter_count as u8,
            escape_iter_count as u8,
            255,
        ]),
        None => Color([0, 0, 0, 255]),
    }
}

/// map any pixel position to a corresponding complex number given the image resolution and a rectangular area on the complex plane
///
/// # Parameters
/// - `pixel_x`, `pixel_y`: pixel position. (top left pixel is `(0, 0)`)
/// - `image_width`, `image_height`: image resolution  
/// - `center`: center complex point of a rectangle
/// - `bottom_right`: bottom right complex point of a rectangle
///
/// # Returns
/// - [Complex]: A unique complex number calculated from params
pub fn pixel_to_complex(
    pixel_x: usize,
    pixel_y: usize,
    image_width: usize,
    image_height: usize,
    center: Complex<f32>,
    dimensions: Complex<f32>,
) -> Complex<f32> {
    let complex_plane_width = dimensions.re;
    let complex_plane_height = dimensions.im;

    let horizontal_ratio = pixel_x as f32 / image_width as f32;
    let vertical_ratio = pixel_y as f32 / image_height as f32;

    let offset = Complex::new(
        complex_plane_width * horizontal_ratio,
        complex_plane_height * vertical_ratio,
    );

    let top_left = center - dimensions / 2.0;

    top_left + offset
}

pub fn escape_time(
    z0: Complex<f32>,
    mut f: impl FnMut(Complex<f32>) -> Complex<f32>,
    bound: f32,
    iteration_max: usize,
) -> Option<usize> {
    let mut z = z0;
    for n in 0..iteration_max {
        z = f(z);
        if z.norm_sqr() > bound {
            return Some(n);
        }
    }
    None
}

/// Calculate color based on the `escape_time` of the each pixel using parallel iterators
///
/// # Parameters
/// - `image_width`, `image_height`: image resolution  
/// - `origin`: the origin of the viewing rectangular area on the complex plane
/// - `iteration_max`: The amount of iterations to cutoff and consider a point part of the mandelbrot set
///
/// # Returns
/// - `Vec<Color>`: The color data of each pixel serialized by rows
pub fn calculate_mandelbrot_color_data(
    image_width: usize,
    image_height: usize,
    center: Complex<f32>,
    dimensions: Complex<f32>,
    iteration_max: usize,
) -> Vec<Color> {
    (0..image_height)
        .into_par_iter()
        .flat_map(|y| {
            (0..image_width).into_par_iter().map(move |x| {
                // turn pixel position into a specific complex number
                let c = pixel_to_complex(x, y, image_width, image_height, center, dimensions);

                // calculate the mandelbrot equation the specified amount of iterations
                let escape_time = escape_time(Complex::ZERO, |z| z * z + c, 4.0, iteration_max);

                // calculate color of the specific complex number
                escape_time_to_grayscale(escape_time)
            })
        })
        .collect()
}

pub fn escape_time_and_path(
    z0: Complex<f32>,
    mut zn: impl FnMut(Complex<f32>) -> Complex<f32>,
    bound: f32,
    iteration_max: usize,
) -> (Option<usize>, Vec<Complex<f32>>) {
    let mut z = z0;
    let mut zs = vec![z];
    for n in 0..iteration_max {
        z = zn(z);
        zs.push(z);
        if z.norm_sqr() > bound {
            return (Some(n), zs);
        }
    }
    (None, zs)
}

pub fn calculate_mandelbrot_escape_times_and_paths(
    image_width: usize,
    image_height: usize,
    center: Complex<f32>,
    dimensions: Complex<f32>,
    iteration_max: usize,
) -> Vec<(Option<usize>, Vec<Complex<f32>>)> {
    (0..image_height)
        .into_par_iter()
        .flat_map(|y| {
            (0..image_width).into_par_iter().map(move |x| {
                // turn pixel position into a specific complex number
                let c = pixel_to_complex(x, y, image_width, image_height, center, dimensions);

                // calculate the mandelbrot equation the specified amount of iterations
                escape_time_and_path(c, |z| z * z + c, 4.0, iteration_max)
            })
        })
        .collect()
}
