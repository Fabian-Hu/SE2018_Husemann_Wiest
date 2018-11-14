
pub struct Attribut {
	pub name: String,
	pub typ: String,			// Enum
	pub value: String,
}

pub struct ObjectHelper {
	pub object: Object,
	pub count: usize,
}

pub struct AttributHelper {
	pub attribut: Attribut,
	pub count: usize,
}

pub struct RelationHelper{
	pub relation: RelationObject,
	pub count: usize,
}

pub struct RelationObject{
	pub description: String,
	pub typ: String,			// Enum
	
	pub from: String,
	pub to: String,
}

pub struct Object {
	pub name: String,
	pub attributes: Vec<Attribut>, 
}

pub fn printRelation(rela: &RelationObject,objList: &Vec<Object>) {
	println!("RalationDescription: {}",rela.description);
	println!("Ralationtyp: {}",rela.typ);
	
	for obj in objList{
		if obj.name == rela.from {
			println!("\nFrom: ");
			printObject(&obj);
		}
	}

	for obj in objList{
		if obj.name == rela.to {
			println!("\nTo: ");
			printObject(&obj);
		}
		
	}
	
}

pub fn printObject(obj: &Object) {
	println!("Objectname: {}",obj.name);
	
	for attr in obj.attributes.iter() {
		println!("Attributname: {}",attr.name);
		println!("Attributtyp: {}",attr.typ);
		println!("Attributvalue: {}",attr.value);
	}
	println!("");	
}


//test
