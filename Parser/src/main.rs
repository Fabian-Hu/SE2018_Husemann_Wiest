use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
mod lib;
mod draw;

fn createFunction(line: &Vec<&str>,lineCount: &usize) -> lib::FunctionHelper {
	let mut lineCount2 = *lineCount;
	let mut funcName = "";
	let mut funcParameter = "";
	let mut funcReturn = "";

	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while !line[lineCount2 as usize].contains("/Function") && funcErrorCount == 0 {	
		lineCount2 = lineCount2 + 1;
		if line[lineCount2].contains("Name:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Name") {
					funcName = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("Parameter:") {
			for split in line[lineCount2].split("Parameter: "){
				funcParameter = split;				
			}				  
		}	
		else if line[lineCount2].contains("Return:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Return") {
					funcReturn = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("/Function"){}
		else {
			funcErrorMsg.push_str("Fehler in Funktion an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}					
	}

	if funcName == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Funktion hat keinen Namen!");	
		funcErrorCount = 1;
	}

	let func = lib::Function{
		name: funcName.to_string(),
		parameter: funcParameter.to_string(),
		returnValue: funcReturn.to_string(),
	};

	let functionHelp = lib::FunctionHelper{
		function: func,
		count: lineCount2,
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	
	};

	return functionHelp;
}


fn createRelation(line: &Vec<&str>,lineCount: &usize) -> lib::RelationHelper {
	let mut lineCount2 = *lineCount;
	let mut relaDescription = "";
	let mut relaTyp = "";
	let mut relaFrom = "";
	let mut relaTo = "";

	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while !line[lineCount2 as usize].contains("/Relation") && funcErrorCount == 0 {
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
					relaFrom = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("To:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("To") {
					relaTo = split;	
				}
			}				  
		}
		else if line[lineCount2].contains("/Relation"){}
		else {
			funcErrorMsg.push_str("Fehler in Relation an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}
	}

	if relaTyp == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Relation hat keinen Typ: !");	
		funcErrorCount = 1;
	}

	if relaFrom == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Relation hat keinen From: !");	
		funcErrorCount = 1;
	}

	if relaTo == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Relation hat keinen To: !");	
		funcErrorCount = 1;
	}

	let relationObject = lib::RelationObject {
	 	description: relaDescription.to_string(),
	 	typ: relaTyp.to_string(),			
	
	 	from: relaFrom.to_string(),
	 	to: relaTo.to_string(),
	};

	let relationHelp = lib::RelationHelper{
		relation: relationObject,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};

	return relationHelp;
}

fn createAttribut(line: &Vec<&str>,lineCount: &usize) -> lib::AttributHelper {
	let mut lineCount2 = *lineCount;
	let mut attname = "";
	let mut atttyp = "";
	let mut attvalue = "";

	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while !line[lineCount2 as usize].contains("/Attribut") && funcErrorCount == 0 {
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
		else if line[lineCount2].contains("/Attribut"){}
		else {
			funcErrorMsg.push_str("Fehler in Attribut an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}	
	}

	if attname == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Attribut hat keinen Name: !");	
		funcErrorCount = 1;
	}

	if atttyp == "" && funcErrorCount == 0{
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Attribut hat keinen Typ: !");	
		funcErrorCount = 1;
	}

	let attr = lib::Attribut{
		name: attname.to_string(),
		typ: atttyp.to_string(),
		value: attvalue.to_string(),
	};

	let attributHelp = lib::AttributHelper{
		attribut: attr,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};

	return attributHelp;
}

fn createObject(line: &Vec<&str>,lineCount: &usize) -> lib::ObjectHelper {
	let mut attrs : Vec<lib::Attribut> = vec![];
	let mut funcs : Vec<lib::Function> = vec![];
	let mut lineCount2 = *lineCount;
	let mut objname = "";
	
	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while line[lineCount2 as usize] != "/Object" && funcErrorCount == 0  {
		lineCount2 = lineCount2 + 1;
		if line[lineCount2].contains("Attribut") {
			let attHelper = createAttribut(&line,&lineCount2);
			if attHelper.errorCount != 0 {
				print!("{}",attHelper.errorMsg);
				funcErrorCount = 1;
			}
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
		else if line[lineCount2].contains("Function") {
			let funcHelper = createFunction(&line,&lineCount2);
			if funcHelper.errorCount != 0 {
				print!("{}",funcHelper.errorMsg);
				funcErrorCount = 1;
			}
			funcs.push(funcHelper.function);
			lineCount2 = funcHelper.count;	
									  
		}
		else if line[lineCount2].contains("/Object"){}
		else {
			funcErrorMsg.push_str("Fehler in Object an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}	
	}

	if objname == "" && funcErrorCount == 0 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Das Objekt hat keinen Name: !");	
		funcErrorCount = 1;
	}

	let obj = lib::Object{
		name: objname.to_string(),
		attributes: attrs,
		functions: funcs,
	};

	let objectHelp = lib::ObjectHelper{
		object: obj,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};

	return objectHelp;
}


fn main() {	
	/*
	*  Datei einlesen und überprüfen
	*/

    // Create a path to the desired file
    let path = Path::new("Object.txt");
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
	
	let mut objList: Vec<lib::Object> = vec![];
	let mut relaList: Vec<lib::RelationObject> = vec![];
	let mut errorCounter = 0;

    while lineCount < strings.len() && errorCounter == 0  {
		if strings[lineCount].contains("Object") {
			let objHelper = createObject(&strings,&lineCount);
			if objHelper.errorCount != 0 {
				print!("{}",objHelper.errorMsg);
				errorCounter = 1;
			}
			println!("");
			objList.push(objHelper.object);
			lineCount = objHelper.count;
		}
		else if strings[lineCount].contains("Relation") {
			let relaHelper = createRelation(&strings,&lineCount);
			if relaHelper.errorCount != 0 {
				print!("{}",relaHelper.errorMsg);
				errorCounter = 1;
			}
			relaList.push(relaHelper.relation);
			lineCount = relaHelper.count;
		}
		else {
			print!("Fehler in der Datei an der Zeile: ");
			print!("{} ",&(lineCount+1).to_string());
			println!("{} ",&strings[lineCount].to_string());
			errorCounter = 1;
		}	
		
		lineCount = lineCount + 1;
    }
	if errorCounter == 0 {
		for obj in &objList {
			lib::printObject(&obj);
		}
		for rela in relaList {
			lib::printRelation(&rela, &objList);
		}
	}

	draw::drawObject("Test.png".to_string(),&objList);
}
