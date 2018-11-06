use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod lib;

fn createRelation(line: &Vec<&str>,lineCount: &usize,objList: &Vec<&lib::Object>) -> lib::RelationHelper {
	let mut lineCount2 = *lineCount;
	let mut relaDescription = "";
	let mut relaTyp = "";
	let mut relaFrom;
	let mut relaTo;

	let mut relaFromString = "";
	let mut relaToString = "";

	while !line[lineCount2 as usize].contains("/Relation") {
		lineCount2 = lineCount2 + 1;
		if line[lineCount2].contains("Description:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Description") {
					relaDescription = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("Typ:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Typ") {
					relaTyp = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("From:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("From") {
					relaFromString = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("To:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("To") {
					relaToString = split;	
				}
			}				  
		}
	}
	
	for obj in objList {
		if obj.name.contains(relaFromString) {
			relaFrom = obj;
		}
		if obj.name.contains(relaToString) {
			relaTo = obj;
		}
	}

	let relationObject = lib::RelationObject {
	 	description: relaDescription.to_string(),
	 	typ: relaTyp.to_string(),			
	
	 	from: relaFrom,
	 	to: relaTo,
	};

	let relationHelp = lib::RelationHelper{
		relation: relationObject,
		count: lineCount2,	
	};

	return relationHelp;
}

fn createAttribut(line: &Vec<&str>,lineCount: &usize) -> lib::AttributHelper {
	let mut lineCount2 = *lineCount;
	let mut attname = "";
	let mut atttyp = "";
	let mut attvalue = "";

	while !line[lineCount2 as usize].contains("/Attribut") {
		lineCount2 = lineCount2 + 1;
		if line[lineCount2].contains("Name:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Name") {
					attname = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("Typ:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Typ") {
					atttyp = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("Wert:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Wert") {
					attvalue = split;	
				}
			}				  
		}	
	}

	let attr = lib::Attribut{
		name: attname.to_string(),
		typ: atttyp.to_string(),
		value: atttyp.to_string(),
	};

	let attributHelp = lib::AttributHelper{
		attribut: attr,
		count: lineCount2,	
	};

	return attributHelp;
}

fn createObject(line: &Vec<&str>,lineCount: &usize) -> lib::ObjectHelper {
	let mut attrs = vec![];
	let mut lineCount2 = *lineCount;
	let mut objname = "";
	
	while line[lineCount2 as usize] != "/Object" {
		lineCount2 = lineCount2 + 1;
		if line[lineCount2].contains("Attribut") {
			let attHelper = createAttribut(&line,&lineCount2);
			attrs.push(attHelper.attribut);
			lineCount2 = attHelper.count;
		}
		else if line[lineCount2].contains("Name:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Name") {
					objname = split;	
				}
			}				  
		}		
	}

	let obj = lib::Object{
		name: objname.to_string(),
		attributes: attrs,
	};

	let objectHelp = lib::ObjectHelper{
		object: obj,
		count: lineCount2,	
	};

	return objectHelp;
}


fn main() {	
	/*
	*  Datei einlesen und überprüfen
	*/

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
	
	/*
	*	Jede Zeile in einen Vector<String> pushen
	*/

    let mut lineCount = 0;
    let mut strings = vec![];	
	
    for word in s.lines() {
		strings.push(word);
    }
    
	/*
	*	Jeden Eintrag des Vectors durchgehen
	*/
	
	let mut objList = vec![];

    while lineCount < strings.len()  {
		if strings[lineCount].contains("Object") {
			let obj = createObject(&strings,&lineCount);
			println!("");
			objList.push(obj.object);
			lineCount = obj.count;
		}
		/*else if strings[lineCount].contains("Relation") {
			let relaHelper = createRelation(&strings,&lineCount);
			lib::printRelation(&relaHelper.relation);
			lineCount = relaHelper.count;
		}*/
		lineCount = lineCount + 1;
    }

	for obj in objList {
		lib::printObject(&obj);
	}
}
