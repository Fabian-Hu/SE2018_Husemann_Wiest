use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod lib;
use std::io::BufReader;

fn createObject(line: &Vec<&str>,mut lineCount: &usize) -> lib::ObjectHelpper {
	let attr2 = lib::Attribut{
		name: String::from("Name2"),
		typ: String::from("Typ2"),
		value: String::from("Value2"),
	};
	let attrs = vec![attr2];
	let obj = lib::Object{
		name: String::from("Objectname"),
		attributes: attrs,
	};



	let mut lineCount2 = *lineCount;
	
	println!("{}",lineCount2);
	while line[lineCount2 as usize] != "/Object" {
		lineCount2 = lineCount2 + 1;
		println!("Kein ende {}", lineCount2);
		
	}
	
	let objectHelp = lib::ObjectHelpper{
		object: obj,
		count: lineCount2,	
	};

	return objectHelp;
}


fn main() {	

    // Create a path to the desired file
    let path = Path::new("hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.description()),
        Ok(_) => println!("File Found"),
    }
	
    let mut lineCount = 0;

    let mut strings = vec![];	
	
    for word in s.lines() {
	strings.push(word);
    }
    

    while lineCount < strings.len()  {
	
	println!("{}: {}",lineCount, strings[lineCount]);
	if(strings[lineCount] == "Object") {
		println!("Juhu ein Object");
		let obj = createObject(&strings,&lineCount);
		lineCount = obj.count;
	}
	lineCount = lineCount + 1;
    }

    /*for line in &strings {
	lineCount = lineCount + 1;
	println!("{}: {}",lineCount, line);
	if(line.trim() == "Object") {
		println!("Juhu ein Object");
		let obj = createObject(&strings,&lineCount);
		lineCount = obj.count-1;
	}
    }*/
}


/*let attr = lib::Attribut{
		name: String::from("Name"),
		typ: String::from("Typ"),
		value: String::from("Value"),
	};

	let attr1 = lib::Attribut{
		name: String::from("Name1"),
		typ: String::from("Typ1"),
		value: String::from("Value1"),
	};

	let attr2 = lib::Attribut{
		name: String::from("Name2"),
		typ: String::from("Typ2"),
		value: String::from("Value2"),
	};

	let attrs = vec![attr,attr1,attr2];

	

	

	let obj = lib::Object{
		name: String::from("Objectname"),
		attributes: attrs,
		//relations: relas,
	};

	let rela = lib::RelationObject {
	 	description: String::from("Description"),
	 	typ: String::from("RalaType"),				
	 	from: &obj,
	 	to: &obj,
	};

	lib::printRelation(&rela);*/
