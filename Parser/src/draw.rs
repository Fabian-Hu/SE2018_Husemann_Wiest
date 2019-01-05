use std::path::Path;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use lib;
use std::env;
use std::collections::HashMap;
use draw::image::{Rgb, RgbImage};
use draw::imageproc::rect::Rect;
use draw::imageproc::drawing::{
	draw_text_mut,
	draw_hollow_ellipse_mut,
    draw_cross_mut,
    draw_line_segment_mut,
    draw_hollow_rect_mut,
    draw_filled_rect_mut,
    draw_hollow_circle_mut,
    draw_filled_circle_mut
};

use self::rusttype::FontCollection;
use self::rusttype::Scale;

//https://www.frustfrei-lernen.de/mathematik/lineare-funktion-zwei-punkte.html
pub fn draw_dashed_line_with_filled(image: &mut RgbImage, lokalXFrom: f32, lokalXTo: f32, lokalYFrom: f32, lokalYTo: f32) {
	let black = Rgb([0u8, 0u8, 0u8]);
	let distance = 5.0; 
	let dirVec = vec![(lokalXTo-lokalXFrom),(lokalYTo-lokalYFrom)];
	let vecLenght = ((dirVec[0]*dirVec[0]+dirVec[1]*dirVec[1])).sqrt();
	let unitVec = vec![(dirVec[0] * (distance/vecLenght)),(dirVec[1] * (distance/vecLenght))];
	
	let unitVecLeft = vec![(((dirVec[1]*-1.0)) * (distance/vecLenght*1.5)),((dirVec[0]) * (distance/vecLenght*1.5))];		
	let unitVecRight = vec![(((dirVec[1])) * (distance/vecLenght*1.5)),(((dirVec[0]*-1.0)) * (distance/vecLenght*1.5))];
	let mut newVecLeft = vec![];
	let mut newVecRight = vec![];

	let diffX = lokalXTo - lokalXFrom;
	let diffY = lokalYTo - lokalYFrom;
	let hypotenuse = ((diffX*diffX+diffY*diffY)).sqrt();
	let len = hypotenuse / distance;
	let mut altVec = vec![lokalXFrom,lokalYFrom];
	for x in 0..len as i32{		
		let newVec = vec![((altVec[0]) + unitVec[0]),((altVec[1]) + unitVec[1])];
		if x % 2 > 0 {			
			draw_line_segment_mut(&mut *image, (newVec[0] as f32, newVec[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
		altVec = newVec;
		if x == 3 {
			
			newVecLeft = vec![(altVec[0] + unitVecLeft[0]),(altVec[1] + unitVecLeft[1])];
			newVecRight = vec![(altVec[0] + unitVecRight[0]),(altVec[1] + unitVecRight[1])];

			draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
			draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
	}

	draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);
	draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);	
}


pub fn draw_dashed_line_with(image: &mut RgbImage, lokalXFrom: f32, lokalXTo: f32, lokalYFrom: f32, lokalYTo: f32) {
	let black = Rgb([0u8, 0u8, 0u8]);
	let distance = 5.0; 
	let dirVec = vec![(lokalXTo-lokalXFrom),(lokalYTo-lokalYFrom)];
	let vecLenght = ((dirVec[0]*dirVec[0]+dirVec[1]*dirVec[1])).sqrt();
	let unitVec = vec![(dirVec[0] * (distance/vecLenght)),(dirVec[1] * (distance/vecLenght))];
	
	let unitVecLeft = vec![(((dirVec[1]*-1.0)) * (distance/vecLenght*1.5)),((dirVec[0]) * (distance/vecLenght*1.5))];		
	let unitVecRight = vec![(((dirVec[1])) * (distance/vecLenght*1.5)),(((dirVec[0]*-1.0)) * (distance/vecLenght*1.5))];
	let mut newVecLeft = vec![];
	let mut newVecRight = vec![];

	let diffX = lokalXTo - lokalXFrom;
	let diffY = lokalYTo - lokalYFrom;
	let hypotenuse = ((diffX*diffX+diffY*diffY)).sqrt();
	let len = hypotenuse / distance;
	let mut altVec = vec![lokalXFrom,lokalYFrom];
	for x in 0..len as i32{		
		let newVec = vec![((altVec[0]) + unitVec[0]),((altVec[1]) + unitVec[1])];
		if x % 2 > 0 {			
			draw_line_segment_mut(&mut *image, (newVec[0] as f32, newVec[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
		altVec = newVec;
		if x == 3 {
			
			newVecLeft = vec![(altVec[0] + unitVecLeft[0]),(altVec[1] + unitVecLeft[1])];
			newVecRight = vec![(altVec[0] + unitVecRight[0]),(altVec[1] + unitVecRight[1])];

			draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
			draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
	}

	draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);
	draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);	
}

pub fn draw_dashed_line(image: &mut RgbImage, lokalXFrom: f32, lokalXTo: f32, lokalYFrom: f32, lokalYTo: f32) {
	let black = Rgb([0u8, 0u8, 0u8]);
	let distance = 5.0; 
	let dirVec = vec![(lokalXTo-lokalXFrom),(lokalYTo-lokalYFrom)];
	let vecLenght = ((dirVec[0]*dirVec[0]+dirVec[1]*dirVec[1])).sqrt();
	let unitVec = vec![(dirVec[0] * (distance/vecLenght)),(dirVec[1] * (distance/vecLenght))];	

	let diffX = lokalXTo - lokalXFrom;
	let diffY = lokalYTo - lokalYFrom;
	let hypotenuse = ((diffX*diffX+diffY*diffY)).sqrt();
	let len = hypotenuse / distance;
	let mut altVec = vec![lokalXFrom,lokalYFrom];
	for x in 0..len as i32{		
		let newVec = vec![((altVec[0]) + unitVec[0]),((altVec[1]) + unitVec[1])];
		if x % 2 > 0 {			
			draw_line_segment_mut(&mut *image, (newVec[0] as f32, newVec[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
		altVec = newVec;	
	}
}

pub fn draw_line_with(image: &mut RgbImage, lokalXFrom: f32, lokalXTo: f32, lokalYFrom: f32, lokalYTo: f32) {		
	let black = Rgb([0u8, 0u8, 0u8]);	
	draw_line_segment_mut(&mut *image, (lokalXFrom, lokalYFrom), (lokalXTo, lokalYTo), black);
	let distance = 5.0; 
	let dirVec = vec![(lokalXTo-lokalXFrom),(lokalYTo-lokalYFrom)];
	let vecLenght = ((dirVec[0]*dirVec[0]+dirVec[1]*dirVec[1])).sqrt();
	let unitVec = vec![(dirVec[0] * (distance/vecLenght)),(dirVec[1] * (distance/vecLenght))];
	
	let unitVecLeft = vec![(((dirVec[1]*-1.0)) * (distance/vecLenght*1.5)),((dirVec[0]) * (distance/vecLenght*1.5))];		
	let unitVecRight = vec![(((dirVec[1])) * (distance/vecLenght*1.5)),(((dirVec[0]*-1.0)) * (distance/vecLenght*1.5))];
	let mut newVecLeft = vec![];
	let mut newVecRight = vec![];

	let diffX = lokalXTo - lokalXFrom;
	let diffY = lokalYTo - lokalYFrom;
	let hypotenuse = ((diffX*diffX+diffY*diffY)).sqrt();
	let len = hypotenuse / distance;
	let mut altVec = vec![lokalXFrom,lokalYFrom];
	for x in 0..len as i32{		
		let newVec = vec![((altVec[0]) + unitVec[0]),((altVec[1]) + unitVec[1])];
		altVec = newVec;
		if x == 3 {			
			newVecLeft = vec![(altVec[0] + unitVecLeft[0]),(altVec[1] + unitVecLeft[1])];
			newVecRight = vec![(altVec[0] + unitVecRight[0]),(altVec[1] + unitVecRight[1])];

			draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
			draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (altVec[0] as f32, altVec[1] as f32), black);
		}
	}

	draw_line_segment_mut(&mut *image, (newVecLeft[0] as f32, newVecLeft[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);
	draw_line_segment_mut(&mut *image, (newVecRight[0] as f32, newVecRight[1] as f32), (lokalXFrom as f32, lokalYFrom as f32), black);	
}


pub fn drawClassDiagram(path: String, objList: &Vec<lib::Object>, relaList: &Vec<lib::RelationObject>) {
	let path = Path::new(&path);
    let white = Rgb([255u8, 255u8, 255u8]);
	let red = Rgb([255u8, 0u8, 0u8]);
	let black = Rgb([0u8, 0u8, 0u8]);

	let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let fontHeight = 16.2;
	let scale = Scale { x: fontHeight * 1.3, y: fontHeight };

	let mut x:f32 = 60.0;
	let mut y:f32 = 10.0;
	let lineHeight:f32 = 22.0;
	let charLenght:f32 = 14.0;
    let mut xCordinate = HashMap::new();	
    let mut yCordinate = HashMap::new();

	let mut image = RgbImage::new(800, 800);
	// Background
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(800, 800), white);
	for obj in objList.iter() {
		 // Draw a hollow rect within bounds
		let mut length:f32 = x;
		let mut height:f32 = y;
		let mut yCordinateFunction;
    	
		// TEXT für Objectname	
		draw_text_mut(&mut image, black, (x+lineHeight/4.0) as u32, (y+lineHeight/6.0) as u32, scale, &font, &obj.name.to_string());

		height +=4.0;
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
			if attrString.len() as f32 * charLenght > length as f32 {
				
				length = attrString.len() as f32 * charLenght;
			}
			//Komplettes Attribute printen;
			draw_text_mut(&mut image, black, (x+lineHeight/4.0) as u32, (height+lineHeight/6.0) as u32, scale, &font, &attrString.to_string());
		}
 
		yCordinateFunction = height + lineHeight;
		height += 4.0;
		
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
			if funcString.len() as f32 * charLenght > length as f32 {
				
				length = funcString.len() as f32 * charLenght;
			}
			//Komplettes Attribute printen;
			draw_text_mut(&mut image, black, (x+lineHeight/4.0) as u32, (height+lineHeight/6.0) as u32, scale, &font, &funcString.to_string());
		} 
		// Box
		draw_hollow_rect_mut(&mut image, Rect::at(x as i32,y as i32).of_size((length-x) as u32, (height-y+lineHeight) as u32), black);
		//Line zwischen Name und Attribut
		draw_line_segment_mut(&mut image, (x, y+lineHeight), (length-1.0 , (y+lineHeight)), black);
		// Line zwischen Attribut und Funktion
		draw_line_segment_mut(&mut image, (x, yCordinateFunction as f32), (length-1.0, yCordinateFunction as f32), black);
		

		xCordinate.insert(obj.name.to_string(),(x/2.0+length)/2.0);
		yCordinate.insert(obj.name.to_string(),(height-y+lineHeight)/2.0+y);
		
		y = height+lineHeight+20.0;
	}
	
	for rela in relaList.iter() {
		let mut lokalXTo = 0.0;
		let mut lokalYTo = 0.0;
		let mut lokalXFrom = 0.0;
		let mut lokalYFrom = 0.0;
		match xCordinate.get(&rela.to) {
        	Some(&cordinate) => lokalXTo = cordinate,
        	_ => println!(),
    	}
		match yCordinate.get(&rela.to) {
        	Some(&cordinate) => lokalYTo = cordinate,
        	_ => println!(),
    	}
		match xCordinate.get(&rela.from) {
        	Some(&cordinate) => lokalXFrom = cordinate,
        	_ => println!(),
    	}
		match yCordinate.get(&rela.from) {
        	Some(&cordinate) => lokalYFrom = cordinate,
        	_ => println!(),
    	}

		match rela.typ{
			lib::RelaTyp::Vererbung => draw_line_with(&mut image,lokalXFrom, lokalXTo, lokalYFrom ,lokalYTo),
			lib::RelaTyp::Kennt => draw_line_segment_mut(&mut image, (lokalXFrom, lokalYFrom), (lokalXTo, lokalYTo), black),
			lib::RelaTyp::Abhaengigkeit => draw_dashed_line_with(&mut image,lokalXFrom, lokalXTo, lokalYFrom ,lokalYTo),
			_ => println!("Es ist ein Fehler beim Zeichnen bei den Relationen"),
		}		
		
	}

	image.save(path).unwrap();
}

pub fn drawUseCaseDiagram(path: String,akteur: &Vec<lib::Akteur>, system: &Vec<lib::System>) {
	let path = Path::new(&path);
	let black = Rgb([0u8, 0u8, 0u8]);	
    let white = Rgb([255u8, 255u8, 255u8]);

	let font = Vec::from(include_bytes!("DejaVuSans.ttf") as &[u8]);
	let font = FontCollection::from_bytes(font).unwrap().into_font().unwrap();
	let fontHeight = 16.2;
	let scale = Scale { x: fontHeight * 1.3, y: fontHeight };	
	let charLenght:i32 = 14;

	let mut xCordinate = vec![];
    let mut yCordinate = vec![];

	let mut image = RgbImage::new(1600, 1600);
	// Background
	draw_filled_rect_mut(&mut image, Rect::at(0, 0).of_size(1600, 1600), white);

	let sys = &system[0];
	let akt = &akteur[0];
	let mut lokalX = 300;
	let mut lokalY = 220;
	
	let mut boxHeight = 40;
	let mut boxLenght = 0;

	//draw_hollow_ellipse_mut(&mut image, (100, 100), 140, 80, black);
	for useCase in &sys.useCases{	
		let lenght = (useCase.description.len() as i32) * charLenght;
		draw_text_mut(&mut image, black, lokalX as u32, lokalY as u32 , scale, &font, &useCase.description);
		lokalY += charLenght/2;
		draw_hollow_ellipse_mut(&mut image, (lokalX+lenght/3, lokalY), (lenght/2) as i32, 50, black);
			
		xCordinate.push(lokalX);
		yCordinate.push(lokalY);
		lokalY += 100;

		boxHeight += 120;
		if (useCase.description.len() as i32 * charLenght) > boxLenght {
			boxLenght = useCase.description.len() as i32 * charLenght;
		}
	}
	// System
	draw_hollow_rect_mut(&mut image, Rect::at(200, 100).of_size(((boxLenght + 80) as u32),boxHeight), black);

	
	draw_text_mut(&mut image, black, ((200 + (boxLenght + 80)/2)- (sys.name.len() as i32 * charLenght)/2) as u32 , 130 , scale, &font, &sys.name);
	// Akteur
   	draw_hollow_circle_mut(&mut image, (100, (100 + (boxHeight/2)) as i32), 30, black);
	
	draw_text_mut(&mut image, black, 70, ((100 + (boxHeight/2))-50) as u32 , scale, &font, &akt.name);

	for x in 0..xCordinate.len(){
		draw_line_segment_mut(&mut image, (100 as f32, (100 + (boxHeight/2)) as f32), (xCordinate[x] as f32, yCordinate[x] as f32), black);
	}
	
	image.save(path).unwrap();
}
