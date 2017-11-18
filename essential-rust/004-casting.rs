// casting types in Rust


fn main() {
	let points: i32 = 10;
	let pinumb: f32 = 3.1415;
	let mut saved_points: u32;

	saved_points = points as u32;
	println!("Casting an i32 to an u32 | {0} -> {1}", points, saved_points);

	saved_points = pinumb as u32;
	println!("Casting a f32 to an u32 truncates | {0} -> {1}", pinumb, saved_points);

}

/*

	Produces the following output:

	Casting an i32 to an u32 | 10 -> 10
	Casting a f32 to an u32 truncates | 3.1415 -> 3

*/
