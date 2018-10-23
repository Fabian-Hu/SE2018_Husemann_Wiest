use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod lib;
use std::io::BufReader;

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
	
    for word in s.split_whitespace() {
			strings.push(word);
    }
    
		println!("{}",strings[0]);
    for word in strings {
			lineCount = lineCount + 1;
			println!("{}: {}",lineCount, word);
    }
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

	

	//let relas = vec![];

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
