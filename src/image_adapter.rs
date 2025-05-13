use super::Color;
use image::{ImageBuffer, Rgb, RgbImage};

impl From<Color> for Rgb<u8> {
    fn from(value: Color) -> Self {
        Rgb([value.red(), value.green(), value.blue()])
    }
}
impl From<&Color> for Rgb<u8> {
    fn from(value: &Color) -> Self {
        Rgb([value.red(), value.green(), value.blue()])
    }
}

pub fn colors_to_rgbimage(color_data: &[Color], width: usize, height: usize) -> RgbImage {
    ImageBuffer::from_par_fn(width as u32, height as u32, |x, y| {
        color_data[(y as usize * width) + x as usize].into()
    })
}
