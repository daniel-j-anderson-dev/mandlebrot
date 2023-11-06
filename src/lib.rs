pub mod terminal_input;

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

/// uses raw rgb pixel values to create an `RgbImage` from image crate
pub fn pixels_to_rgbimage(pixels: &[Pixel], width: usize, height: usize) -> image::RgbImage {
    let mut output = image::RgbImage::new(width as u32, height as u32);
    for pixel in pixels {
        *output.get_pixel_mut(pixel.x as u32, pixel.y as u32) = pixel.color.into();
    }
    output
}
/// uses raw rgb pixel values to create an `ColorImage` from epaint crate
pub fn pixels_to_colorimage(pixels: &[Pixel], width: usize, height: usize) -> epaint::ColorImage {
    let mut output = epaint::ColorImage::new([width, height], epaint::Color32::from_rgb(0, 0, 0));
    output.pixels = pixels
        .into_iter()
        .map(|pixel| pixel.color.into())
        .collect();
    output
}

/// # Parameters
/// - `c`: A complex number to be colored depending on its `escape_time`
/// - `iteration_max`: The max number of iterations for `escape_time`
/// 
/// # Returns
/// - `Color`: raw rgb values.<br>
///  Black if not in mandlebrot set or if c escaped imediatly.
///  Otherwise `escape_iter_count` is used for r, g, and b
pub fn complex_to_grayscale(c: num::Complex<f64>, iteration_max: usize) -> Color {
    match escape_time(c, iteration_max) {
        Some(escape_iter_count) => {
            Color(escape_iter_count as u8, escape_iter_count as u8, escape_iter_count as u8)
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
    top_left: num::Complex<f64>,
    bottom_right: num::Complex<f64>,
) -> num::Complex<f64>
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
    
    num::Complex::new(real, imaginary)
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
pub fn escape_time(c: num::Complex<f64>, iteration_max: usize) -> Option<usize> {
    let mut z = num::Complex::new(0.0, 0.0);
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
    origin: num::Complex<f64>,
    iteration_max: usize,
) -> Vec<Pixel> {
    use rayon::iter::{
        ParallelIterator,
        IntoParallelIterator
    };
    let top_left = origin + num::Complex::new(-2.0, 1.2).scale(scale_factor);
    let bottom_right = origin + num::Complex::new(0.5, -1.2).scale(scale_factor);
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

#[cfg(test)]
mod tests {
    use super::*;
    use terminal_input::{get_complex, get_num};
    use std::time::Instant;
    
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = 1080;
    const SCALE_FACTOR: f64 = 0.5625;
    const ORIGIN: num::Complex::<f64> = num::Complex::new(0.0, 0.0);
    const ITERATION_MAX: usize = 500;

    #[test]
    pub fn default() -> Result<(), Box<dyn std::error::Error>> {
        let grand_start = Instant::now();
            
        let start = Instant::now();
        
        let pixels = calculate_pixel_data(IMAGE_WIDTH, IMAGE_HEIGHT, SCALE_FACTOR, ORIGIN, ITERATION_MAX);
            
        let end = Instant::now();
        let pixel_delta = end - start;
            
        let start = Instant::now();
        let output = pixels_to_rgbimage(&pixels, IMAGE_WIDTH, IMAGE_HEIGHT);
        let end = Instant::now();
        let image_delta = end - start;
        
        output.save(format!("mandlebrot_{}x{}_{}_iter.png", IMAGE_WIDTH, IMAGE_HEIGHT, ITERATION_MAX))?;
            
        let end = Instant::now();
        let save_delta = end - start;
        let grand_delta = end - grand_start;
    
        println!("Resolution: {}x{}", IMAGE_WIDTH, IMAGE_HEIGHT);
        println!("Number of itterations: {}", ITERATION_MAX);
        println!("Grand total: {:?}", grand_delta);
        println!("Calculating pixel colors: {:?}", pixel_delta);
        println!("Copying raw pixels into image: {:?}", image_delta);
        println!("Saving: {:?}", save_delta);
    
        return Ok(());
    }
    
    #[test]
    pub fn terminal() -> Result<(), Box<dyn std::error::Error>> {
        let grand_start = Instant::now();
        
        let image_width = get_num("Enter image width: ")?;
        let image_height = get_num("Enter image height: ")?;
        let scale_factor = get_num("Enter scale factor: ")?;
        let origin = get_complex("Enter a complex number to be the origin of the image.\n")?;
        let iteration_max = get_num("Enter max number of iterations: ")?;
            
        let start = Instant::now();
        
        let pixels = calculate_pixel_data(image_width, image_height, scale_factor, origin, iteration_max);
            
        let end = Instant::now();
        let pixel_delta = end - start;
    
            
        let start = Instant::now();
       
        let output = pixels_to_rgbimage(&pixels, image_width, image_height);
      
        let end = Instant::now();
        let image_delta = end - start;
        
        output.save(format!("mandlebrot_{}x{}_{}_iter.png", image_width, image_height, iteration_max))?;
    
        let end = Instant::now();
        let save_delta = end - start;
        let grand_delta = end - grand_start;
    
        println!("Resolution: {}x{}", image_width, image_height);
        println!("Number of itterations: {}", iteration_max);
        println!("Grand total: {:?}", grand_delta);
        println!("Time calculating pixel colors: {:?}", pixel_delta);
        println!("Time copying pixels into image: {:?}", image_delta);
        println!("Save time: {:?}", save_delta);
    
        return Ok(());
    }
}