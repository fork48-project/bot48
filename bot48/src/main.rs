use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Point {
	x: i32,
	y: i32
}

fn main() {
	let point = Point { x: 123, y: 456 };
	
	let serialized = bincode::serialize(&point).unwrap();//serde_json::to_string(&point).unwrap();
	
	println!("serialized = {:?}", serialized);
	
	let deserialized: Point = bincode::deserialize(&serialized[..]).unwrap();//serde_json::from_str(&serialized).unwrap();
	
	println!("deserialized = {:?}", deserialized);
}
