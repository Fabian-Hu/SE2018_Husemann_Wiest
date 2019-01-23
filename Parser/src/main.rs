extern crate gio;
extern crate gtk;

use std::env::args;
use std::io::prelude::*;
use std::io::BufReader;
use gio::prelude::*;
use gtk::prelude::*;
use gtk::Builder;
use std::error::Error;
use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::path::Path;
use gtk::{ButtonsType, DialogFlags, MessageType, MessageDialog, Window};
mod lib;
mod draw;


#[macro_export]
macro_rules! upgrade_weak {
    ($x:ident, $r:expr) => {{
        match $x.upgrade() {
            Some(o) => o,
            None => return $r,
        }
    }};
    ($x:ident) => {
        upgrade_weak!($x, ())
    };
}
pub fn build_ui(application: &gtk::Application) {
    let glade_src = include_str!("text_viewer.glade");
    let builder = Builder::new();
    builder.add_from_string(glade_src).expect("Couldn't add from string");

    let window: gtk::ApplicationWindow = builder.get_object("window").expect("Couldn't get window");
    window.set_application(application);
    window.set_title("Parser");
    let open_button: gtk::ToolButton = builder.get_object("open_button")
                                              .expect("Couldn't get builder");
    let text_view: gtk::TextView = builder.get_object("text_view")
                                          .expect("Couldn't get text_view");
    let parse_button: gtk::Button = builder.get_object("parseButton")
                                              .expect("Couldn't get builder");
    let mut parsedImage: gtk::Image = builder.get_object("parsedImage")
                                              .expect("Couldn't get builder");
    let imageParsed=gtk::Image::new_from_file("test.png");
    
    let mut textFromDatei= String::new();
    let mut textFromDateiClone= textFromDatei.clone();

    let window_weak = window.downgrade();
    let text_view_copy=text_view.clone();
    open_button.connect_clicked(move |_| {
        let window = upgrade_weak!(window_weak);
        let file_chooser = gtk::FileChooserDialog::new(
            Some("Open File"), Some(&window), gtk::FileChooserAction::Open);
        file_chooser.add_buttons(&[
            ("Open", gtk::ResponseType::Ok.into()),
            ("Cancel", gtk::ResponseType::Cancel.into()),
        ]);
        if file_chooser.run() == gtk::ResponseType::Ok.into() {
            let filename = file_chooser.get_filename().expect("Couldn't get filename");
            let file = File::open(&filename).expect("Couldn't open file");
            let mut contents = String::new();
            let mut reader = BufReader::new(file);		 
            let _ = reader.read_to_string(&mut contents);
            text_view.get_buffer().expect("Couldn't get window").set_text(&contents);
	    //parseString(&contents);
	}
        file_chooser.destroy();
    });
    let counter=0;
    parse_button.connect_clicked(move |_| {
    	let startiter=text_view_copy.get_buffer().expect("Couldn't get window").get_start_iter();
    	let enditer=text_view_copy.get_buffer().expect("Couldn't get window").get_end_iter();
    	let textTest=text_view_copy.get_buffer().expect("Couldn't get window").get_text(&startiter, &enditer, false);
    	//println!("{:#?}", textTest.unwrap());
        parseString(&textTest.unwrap());


    });
    parsedImage=imageParsed;
    window.connect_delete_event(|win, _| {
        win.destroy();
        Inhibit(false)
    });
    window.show_all();
}

fn errorMessage(s: &String) {
         MessageDialog::new(None::<&Window>,
                       DialogFlags::empty(),
                       MessageType::Info,
                       ButtonsType::Ok,
                       s).run();
}


fn parseString(s: &String) {
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
	let mut diagramTyp = "";
	let mut akteur: Vec<lib::Akteur> = vec![]; 
	let mut system: Vec<lib::System> = vec![];

	if strings[0].contains("Klassendiagramm") {
			diagramTyp = "Klassendiagramm";
			lineCount = lineCount + 1;
	}else if strings[0].contains("Usecasediagramm"){
			diagramTyp = "Usecasediagramm";
			lineCount = lineCount + 1;
	}
	
    while lineCount < strings.len() && errorCounter == 0  {
		if diagramTyp ==  "Klassendiagramm" {
			if strings[lineCount].contains("Object") {
				let objHelper = createObject(&strings,&lineCount);
				if objHelper.errorCount != 0 {
					//print!("{}",objHelper.errorMsg);
					errorMessage(&objHelper.errorMsg);
					errorCounter = 1;
				}
				println!("");
				objList.push(objHelper.object);
				lineCount = objHelper.count;
			}
			else if strings[lineCount].contains("Relation") {
				let relaHelper = createRelation(&strings,&lineCount,&objList);
				if relaHelper.errorCount != 0 {
					//print!("{}",relaHelper.errorMsg);
					errorMessage(&relaHelper.errorMsg); 
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
		} else if diagramTyp == "Usecasediagramm" {
			if strings[lineCount].contains("Akteur") {
				let aktHelper = createAkteur(&strings,&lineCount);
				if aktHelper.errorCount != 0 {
					//print!("{}",aktHelper.errorMsg);
					errorMessage(&aktHelper.errorMsg);
					errorCounter = 1;
				}
				println!("");
				akteur.push(aktHelper.akteur);
				lineCount = aktHelper.count;
			}else if strings[lineCount].contains("System") {
				let sysHelper = createSystem(&strings,&lineCount);
				if sysHelper.errorCount != 0 {
					//print!("{}",sysHelper.errorMsg);
					errorMessage(&sysHelper.errorMsg);
					errorCounter = 1;
				}
				println!("");
				system.push(sysHelper.system);				
				lineCount = sysHelper.count;
			}else {
				print!("Fehler in der Datei an der Zeile: ");
				print!("{} ",&(lineCount+1).to_string());
				println!("{} ",&strings[lineCount].to_string());
				errorCounter = 1;
			}
		}
		lineCount = lineCount + 1;
    }
	

	if errorCounter == 0 {	
		if diagramTyp == "Klassendiagramm" {
			calculateWeighting(&mut objList,&relaList);
			println!();	
			for obj in objList.iter() {
				println!("{} Gewicht: {}",obj.name,obj.weighting);
			}		
			draw::drawClassDiagram("Klassendiagramm.png".to_string(),&mut objList,&relaList);
		}else if diagramTyp == "Usecasediagramm"{
			draw::drawUseCaseDiagram("UseCaseDiagramm.png".to_string(),&akteur,&system);
		}
	}

	println!("Der Parser war Erfolgreich");
}

fn calculateWeighting(objList: &mut Vec<lib::Object>, relaList: &Vec<lib::RelationObject>) {	
	for rela in relaList.iter() {
		let mut tempFromIndex = 0;
		let mut tempToIndex = 0;
		for x in 0..objList.len(){	
			if objList[x].name.to_string() == rela.from.to_string() {
				tempFromIndex = x;
			}

			if objList[x].name.to_string() == rela.to.to_string() {
				tempToIndex = x;
			}
		}

		let mut tempName = objList[tempToIndex].name.clone();
		objList[tempFromIndex].addChild(tempName);

		if objList[tempFromIndex].weighting == 1 && objList[tempToIndex].weighting == 1 {
			objList[tempFromIndex].setWeighting(10);
			objList[tempToIndex].setWeighting(0);
		}else if objList[tempFromIndex].weighting != 1 && objList[tempToIndex].weighting == 1 {
			let mut tempValue = objList[tempFromIndex].weighting.clone();		
			objList[tempToIndex].setWeighting(tempValue - 10);
		}else if objList[tempFromIndex].weighting == 1 && objList[tempToIndex].weighting != 1 {
			let mut tempValue = objList[tempToIndex].weighting.clone();
			objList[tempFromIndex].setWeighting(tempValue+10);
		}else if objList[tempFromIndex].weighting != 1 && objList[tempToIndex].weighting != 1 {
			let mut tempFromW = objList[tempFromIndex].weighting.clone();
			let mut tempToW = objList[tempToIndex].weighting.clone();
			let mut tempDiff = tempFromW - tempToW - 10;		
			objList[tempToIndex].addWeighting(tempDiff);
		
			addAllWeight(objList,tempDiff,tempToIndex);
		}

		println!("{} Aktuelles Gewicht: {}",objList[tempFromIndex].name,objList[tempFromIndex].weighting);
		println!("{} Aktuelles Gewicht: {}",objList[tempToIndex].name,objList[tempToIndex].weighting);
	}		
}

fn addAllWeight(objList: &mut Vec<lib::Object>,value: i32,startIndex: usize){
	let tempChildName = objList[startIndex].child.clone();
	for childName in tempChildName.iter(){
		for y in 0..objList.len(){	
			if objList[y].name == *childName {
				objList[y].addWeighting(value);
				addAllWeight(objList,value,y);
			}
		}
	}
}

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
			funcErrorMsg.push_str(" Fehler in Funktion an der Zeile: ");
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


fn createRelation(line: &Vec<&str>,lineCount: &usize, objList: &Vec<lib::Object>) -> lib::RelationHelper {
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
			funcErrorMsg.push_str(" Fehler in Relation an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}
	}
	
	let mut lokalErrorCount = 1;
	for obj in objList.iter() {
		if obj.name == relaTo {
			lokalErrorCount = 0
		}	
	}
	if lokalErrorCount == 1 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Das angegebene To ist nicht vorhanden");	
		funcErrorCount = 1;
	}

	let mut lokalErrorCount2 = 1;
	for obj in objList.iter() {
		if obj.name == relaFrom {
			lokalErrorCount2 = 0
		}	
	}
	if lokalErrorCount2 == 1 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Das angegebene From ist nicht vorhanden");	
		funcErrorCount = 1;
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

	let mut rela_typ = lib::RelaTyp::Fehler;

	if relaTyp.contains("Vererbung"){
		rela_typ = lib::RelaTyp::Vererbung;
	}else if relaTyp.contains("Kennt"){
		rela_typ = lib::RelaTyp::Kennt;
	}else if relaTyp.contains("Abhängigkeit"){
		rela_typ = lib::RelaTyp::Abhaengigkeit;
	}else if relaTyp.contains("Aggregation"){
		rela_typ = lib::RelaTyp::Aggregation;
	}

	let relationObject = lib::RelationObject {
	 	description: relaDescription.to_string(),
	 	typ: rela_typ,			
	
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
	let mut children : Vec<String> = vec![];
	let mut obj = lib::Object{
		name: objname.to_string(),
		attributes: attrs,
		functions: funcs,
		weighting: 1,
		child: children,
	};

	let objectHelp = lib::ObjectHelper{
		object: obj,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};

	return objectHelp;
}

fn createAkteur(line: &Vec<&str>,lineCount: &usize) -> lib::AkteurHelper {
	let mut lineCount2 = *lineCount;
	let mut aktName = "";	
	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while !line[lineCount2 as usize].contains("/Akteur") && funcErrorCount == 0 {
		lineCount2 = lineCount2 + 1;
	 	if line[lineCount2].contains("Name:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Name") {
					aktName = split;	
				}
			}				  
		}else if line[lineCount2].contains("/Akteur"){}
		else {
			funcErrorMsg.push_str("Fehler im Akteur an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}			
	}
	
	if aktName == "" && funcErrorCount == 0 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Der Akteur hat keinen Name: !");	
		funcErrorCount = 1;
	}

	let akt = lib::Akteur{
		name: aktName.to_string(),
	};

	let akteurHelp = lib::AkteurHelper{
		akteur: akt,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};
	return akteurHelp;
}

fn createUseCase(line: &Vec<&str>,lineCount: &usize) -> lib::UseCaseHelper {
	let mut lineCount2 = *lineCount;
	let mut description = "";	
	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();

	while !line[lineCount2 as usize].contains("/Usecase") && funcErrorCount == 0 {
		lineCount2 = lineCount2 + 1;
	 	if line[lineCount2].contains("Description:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Description") {
					description = split;	
				}
			}				  
		}else if line[lineCount2].contains("/Usecase"){}
		else {
			funcErrorMsg.push_str("Fehler in Usecase an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}			
	}
	
	if description == "" && funcErrorCount == 0 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Das Usecase hat keine Description: !");	
		funcErrorCount = 1;
	}

	let useCase = lib::UseCase{
		description: description.to_string(),
	};

	let useHelp = lib::UseCaseHelper{
		useCase: useCase,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};
	return useHelp;
}

fn createSystem(line: &Vec<&str>,lineCount: &usize) -> lib::SystemHelper {
	let mut lineCount2 = *lineCount;
	let mut sysName = "";	
	let mut funcErrorCount = 0;
	let mut funcErrorMsg  = String::new();
	let mut useCaseList: Vec<lib::UseCase> = vec![];

	while !line[lineCount2 as usize].contains("/System") && funcErrorCount == 0 {
		lineCount2 = lineCount2 + 1;
	 	if line[lineCount2].contains("Name:") {
			for split in line[lineCount2].split(": "){
				if !split.contains("Name") {
					sysName = split;	
				}
			}				  
		}else if line[lineCount2].contains("Usecase") {
			let useHelper = createUseCase(&line,&lineCount2);
			if useHelper.errorCount != 0 {
				print!("{}",useHelper.errorMsg);
				funcErrorCount = 1;
			}
			useCaseList.push(useHelper.useCase);
			lineCount2 = useHelper.count;		  
		}else if line[lineCount2].contains("/System"){
		}else {
			funcErrorMsg.push_str("Fehler in Usecase an der Zeile: ");
			funcErrorMsg.push_str(&(lineCount2+1).to_string());
			funcErrorMsg.push_str(" ");
			funcErrorMsg.push_str(&line[lineCount2].to_string());
			funcErrorCount = 1;
		}	
	}
	
	if sysName == "" && funcErrorCount == 0 {
		funcErrorMsg.push_str("Zeile: ");
		funcErrorMsg.push_str(&lineCount2.to_string());
 		funcErrorMsg.push_str(" Fehler: Das Objekt hat keinen Name: !");	
		funcErrorCount = 1;
	}

	let sys = lib::System{
		name: sysName.to_string(),
		useCases: useCaseList,
	};

	let systemHelp = lib::SystemHelper{
		system: sys,
		count: lineCount2,	
		errorCount: funcErrorCount,			
		errorMsg: funcErrorMsg.to_string(),
	};
	return systemHelp;
}

fn main() {
    let application = gtk::Application::new("com.github.text_viewer",
                                            gio::ApplicationFlags::empty())
                                       .expect("Initialization failed...");

    application.connect_startup(|app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
