use std::path::Path;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use lib;
use std::env;
use draw::image::{Rgb, RgbImage};
use draw::imageproc::rect::Rect;
use draw::imageproc::drawing::{
	draw_text_mut,
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut
};


pub fn drawObject(path: String, objList: &Vec<lib::Object>) {
	let path = Path::new(&path);

    let red   = Rgb([255u8, 0u8,   0u8]);
    let green = Rgb([0u8,   255u8, 0u8]);
    let blue  = Rgb([0u8,   0u8,   255u8]);
    let white = Rgb([255u8, 255u8, 255u8]);
	let black = Rgb([0u8, 0u8, 0u8]);

	let mut x = 60;
	let mut y = 10;

	let mut image = RgbImage::new(800, 800);
	// Background
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(800, 800), white);

	for obj in objList.iter() {
		 // Draw a hollow rect within bounds
    	draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(160, 80), black);
   	 	draw_line_segment_mut(&mut image, (x as f32, y as f32), (240f32, 110f32), black);

		x += 180;
		y += 100;
	} 

	image.save(path).unwrap();
}
