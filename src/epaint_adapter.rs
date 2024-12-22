use super::Color;
use epaint::{Color32, ColorImage};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

impl From<Color> for epaint::Color32 {
    fn from(value: Color) -> Self {
        Color32::from_rgb(value.0, value.1, value.2)
    }
}
impl From<&Color> for epaint::Color32 {
    fn from(value: &Color) -> Self {
        Color32::from_rgb(value.0, value.1, value.2)
    }
}

pub fn colors_to_colorimage(color_data: &[Color], width: usize, height: usize) -> ColorImage {
    let mut output = ColorImage::new([width, height], Color32::from_rgb(0, 0, 0));
    output.pixels = color_data.into_par_iter().map(Color32::from).collect();
    output
}
