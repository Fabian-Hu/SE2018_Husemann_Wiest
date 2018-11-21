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

use self::rusttype::FontCollection;
use self::rusttype::Scale;

pub fn drawObject(path: String, objList: &Vec<lib::Object>) {
	let path = Path::new(&path);

    let red   = Rgb([255u8, 0u8,   0u8]);
    let green = Rgb([0u8,   255u8, 0u8]);
    let blue  = Rgb([0u8,   0u8,   255u8]);
    let white = Rgb([255u8, 255u8, 255u8]);
	let black = Rgb([0u8, 0u8, 0u8]);

	let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let fontHeight = 16.2;
	let scale = Scale { x: fontHeight * 1.3, y: fontHeight };

	let mut x:i32 = 60;
	let mut y:i32 = 10;
	let lineHeight:i32 = 22;
	let charLenght:i32 = 14;
    let mut xCordinate = vec![];	
    let mut yCordinate = vec![];

	let mut image = RgbImage::new(800, 800);
	// Background
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(800, 800), white);
	for obj in objList.iter() {
		 // Draw a hollow rect within bounds
		let mut length:i32 = x;
		let mut height:i32 = y;
		let mut yCordinateFunction;
    	
		// TEXT für Objectname	
		draw_text_mut(&mut image, black, (x+lineHeight/4) as u32, (y+lineHeight/6) as u32, scale, &font, &obj.name.to_string());

		height +=4;
		for attr in obj.attributes.iter(){
			height += lineHeight;
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
			draw_text_mut(&mut image, black, (x+lineHeight/4) as u32, (height+lineHeight/6) as u32, scale, &font, &attrString.to_string());
		} 
		yCordinateFunction = height + lineHeight;
		height += 4;
		
   	 	for func in obj.functions.iter(){
			height += lineHeight;
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
			draw_text_mut(&mut image, black, (x+lineHeight/4) as u32, (height+lineHeight/6) as u32, scale, &font, &funcString.to_string());
		} 
		// Box
		draw_hollow_rect_mut(&mut image, Rect::at(x, y).of_size((length-x) as u32, (height-y+lineHeight) as u32), black);
		//Line zwischen Name und Attribut
		draw_line_segment_mut(&mut image, (x as f32, (y+lineHeight) as f32), ((length-1) as f32, (y+lineHeight) as f32), black);
		// Line zwischen Attribut und Funktion
		draw_line_segment_mut(&mut image, (x as f32, yCordinateFunction as f32), ((length-1) as f32, yCordinateFunction as f32), black);
		

		xCordinate.push((x/2+length)/2);
		yCordinate.push((height-y+lineHeight)/2+y);
		
		y = height+lineHeight+20;
	}
	
	draw_line_segment_mut(&mut image, (xCordinate[0] as f32, yCordinate[0] as f32), (xCordinate[1] as f32, yCordinate[1] as f32), red);
	
	image.save(path).unwrap();
}
