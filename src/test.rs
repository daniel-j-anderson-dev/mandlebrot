#[cfg(test)]
use super::{
    *,
    terminal_input::{
        get_complex,
        get_num
    },
};


#[test]
pub fn default() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;
    let grand_start = Instant::now();
    const IMAGE_WIDTH: usize = 1920;
    const IMAGE_HEIGHT: usize = 1080;
    const SCALE_FACTOR: f64 = 0.5625;
    const ORIGIN: num::Complex::<f64> = num::Complex::new(0.0, 0.0);
    const ITERATION_MAX: usize = 500;
        
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
    use std::time::Instant;
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