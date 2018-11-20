//! An example using the drawing functions. Writes to the user-provided target file.

extern crate image;
extern crate imageproc;

use std::env;
use std::path::Path;
use image::{Rgb, RgbImage};
use imageproc::rect::Rect;
use imageproc::drawing::{
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut
};

fn main() {

    let arg = if env::args().count() == 2 {
            env::args().nth(1).unwrap()
        } else {
            panic!("Please enter a target file path")
        };

    let path = Path::new(&arg);

    let red   = Rgb([255u8, 0u8,   0u8]);
    let green = Rgb([0u8,   255u8, 0u8]);
    let blue  = Rgb([0u8,   0u8,   255u8]);
    let white = Rgb([255u8, 255u8, 255u8]);

    let mut image = RgbImage::new(200, 200);

    // Draw some crosses within bounds
    draw_cross_mut(&mut image, white, 5, 5);
    draw_cross_mut(&mut image, red, 9, 9);
    draw_cross_mut(&mut image, blue, 9, 5);
    draw_cross_mut(&mut image, green, 5, 9);
   

    image.save(path).unwrap();
}
