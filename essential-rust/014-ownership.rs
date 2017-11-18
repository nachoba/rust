

fn main() {
	let name = format!("A String");
	
	helper(name);
	helper(name);
}


fn helper(name: String) {
	println!("{}", name);
}
