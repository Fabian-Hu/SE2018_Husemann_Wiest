pub enum RelaTyp {
	Fehler,
	Vererbung,
	Kennt,
	Abhaengig,	
	Aggregation,
}

pub struct Attribut {
	pub name: String,
	pub typ: String,			// Enum
	pub value: String,
}

pub struct ObjectHelper {
	pub object: Object,
	pub count: usize,
	pub errorCount: usize,				
	pub errorMsg: String,
}

pub struct FunctionHelper {
	pub function: Function,
	pub count: usize,
	pub errorCount: usize,		
	pub errorMsg: String,
}

pub struct AttributHelper {
	pub attribut: Attribut,
	pub count: usize,
	pub errorCount: usize,			
	pub errorMsg: String,
}

pub struct RelationHelper{
	pub relation: RelationObject,
	pub count: usize,
	pub errorCount: usize,			
	pub errorMsg: String,
}

pub struct AkteurHelper{
	pub akteur: Akteur,
	pub count: usize,
	pub errorCount: usize,			
	pub errorMsg: String,
}

pub struct UseCaseHelper{
	pub useCase: UseCase,
	pub count: usize,
	pub errorCount: usize,			
	pub errorMsg: String,
}

pub struct SystemHelper{
	pub system: System,
	pub count: usize,
	pub errorCount: usize,			
	pub errorMsg: String,
}

pub struct RelationObject{
	pub description: String,
	pub typ: RelaTyp,			// Enum
	
	pub from: String,
	pub to: String,
}

pub struct Function {
	pub name: String,
	pub parameter: String, 
	pub returnValue: String, 
}

pub struct drawHelper{
	pub height: f32,
	pub lenght: f32,
}

pub struct objectJoins{
	pub upperJoinX: f32,
	pub upperJoinY: f32,
	pub lowerJoinX: f32,
	pub lowerJoinY: f32,
}

pub struct Object {
	pub name: String,
	pub attributes: Vec<Attribut>, 
	pub functions: Vec<Function>,
	pub weighting: i32,
	pub child: Vec<String>,
}

impl Object {
	pub fn addWeighting(&mut self, value: i32){
		self.weighting += value;
	}

	pub fn subWeighting(&mut self, value: i32){
		self.weighting -= value;
	}

	pub fn setWeighting(&mut self, value: i32){
		self.weighting = value;
	}

	pub fn addChild(&mut self, child: String){
		self.child.push(child);
	}

}

pub struct System {
	pub name: String,
	pub useCases: Vec<UseCase>, 
}

pub struct Akteur {
	pub name: String, 
}

pub struct UseCase {
	pub description: String, 
}

pub fn printRelation(rela: &RelationObject,objList: &Vec<Object>) {
	println!("RalationDescription: {}",rela.description);
	//println!("Ralationtyp: {}",rela.typ);
	
	for obj in objList{
		if obj.name.to_string() == rela.from.to_string() {
			println!("From: ");
			printObject(&obj);
		}
	}

	for obj in objList{
		if obj.name.to_string() == rela.to.to_string() {
			println!("To: ");
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
	if obj.functions.len() != 0 {
		println!("-------------------------------------");
	}
	for func in obj.functions.iter() {
		println!("Funktionsname: {}",func.name);
		println!("Parameter: {}",func.parameter);
		println!("Return: {}",func.returnValue);
	}
	println!("");	
}

