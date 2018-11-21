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

	let mut x:i32 = 60;
	let mut y:i32 = 10;
	let lineHight:i32 = 20;
	let charLenght:i32 = 8;
    let mut xCordinate = vec![];	
    let mut yCordinate = vec![];

	let mut image = RgbImage::new(800, 800);
	// Background
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(800, 800), white);

	for obj in objList.iter() {
		 // Draw a hollow rect within bounds
		let mut length:i32 = 40;
		let mut hight:i32 = 30;
		let mut yCordinateFunction;
    	
		// TEXT für Objectname
		hight +=1;
		for attr in obj.attributes.iter(){
			hight += lineHight;
			let mut attrString = String::new();
			attrString.push_str(&attr.name.to_string());
			attrString.push_str(": ");
			attrString.push_str(&attr.typ.to_string());
			if attr.value != ""{
				attrString.push_str(" = ");
				attrString.push_str(&attr.value.to_string());
			}
			if attrString.len() as i32 * charLenght > length as i32 {
				
				length = attrString.len() as i32 * charLenght;
			}
			//Komplettes Attribute printen;
		} 

		yCordinateFunction = hight;
		hight += 1;
		
   	 	for func in obj.functions.iter(){
			hight += lineHight;
			let mut funcString = String::new();
			funcString.push_str(&func.name.to_string());
			funcString.push_str("(");
			if func.parameter != "" {
				funcString.push_str(&func.parameter.to_string());
			}	
			funcString.push_str(")");
			if func.returnValue != ""{
				funcString.push_str(": ");
				funcString.push_str(&func.returnValue.to_string());
			}
			if funcString.len() as i32 * charLenght > length as i32 {
				
				length = funcString.len() as i32 * charLenght;
			}
			//Komplettes Attribute printen;
		} 
		// Box
		draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size(length as u32, hight as u32), black);
		//Line zwischen Name und Attribut
		draw_line_segment_mut(&mut image, (x as f32, (y+lineHight) as f32), ((x+length) as f32, (y+lineHight) as f32), black);
		// Line zwischen Attribut und Funktion
		draw_line_segment_mut(&mut image, (x as f32, yCordinateFunction as f32), ((x+length) as f32, yCordinateFunction as f32), black);
		

		xCordinate.push(x);
		yCordinate.push(y);
		x += 180;
		y += 100;
	}

		
	image.save(path).unwrap();
}
