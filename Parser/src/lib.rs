
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

pub struct RelationHelper<'a> {
	pub relation: RelationObject<'a>,
	pub count: usize,
}

pub struct RelationObject<'a>{
	pub description: String,
	pub typ: String,			// Enum
	
	pub from: &'a Object,
	pub to: &'a Object,
}

pub struct Object {
	pub name: String,
	pub attributes: Vec<Attribut>, 
}

pub fn printRelation(rela: &RelationObject) {
	println!("RalationDescription: {}",rela.description);
	println!("Ralationtyp: {}",rela.typ);
	println!("\nFrom: ");
	printObject(&rela.from);
	println!("\nTo: ");
	printObject(&rela.to);
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
