/* Primitive Types in Rust
 *
 */


 fn main() {
 	// Booleans: Rust has a built-in boolean type, named bool.
 	//			 It has two values: true and false.
 	let yes: bool = true;
 	let nay		  = false;

 	// Chararacter: The char type represents a single Unicode
 	//				scalar value. You can create a char with ''
 	let two_hearts: char = '💕';

 	// Numeric Types: Rust has a variety of numeric types in a
 	//				  few categories: signed and unsigned, fixed
 	//				  and variable, floating-point and integer.

 	let integer8: i8 	= 12;
 	let integer16: i16  = 32235;
 	let integer32: i32 	= 1_000_000;
 	let integer64: i64 	= 100_000_000_000;

 	// Beside signed integers you have unisigned integers:
 	// u8, u16, u32, and u64.

 	let float32: f32 	= 3.141592;
 	let float64: f64    = 3.1413 / 0.987532;

 	// Finaly Rust also provides types whose particular type
 	// depends on the underlying machine architecture:
 	// isize and usize

 	println!("I am a boolean {}", yes);
 	println!("I'm not a boolean {}", nay);

 	println!("I'm an 8-bit integer {}", integer8);
 	println!("I'm a 16-bit integer {}", integer16);
 	println!("I'm a 32-bit integer {}", integer32);
 	println!("I'm a 64-bit integer {}", integer64);

 	println!("I'm a floating-point 32-bit number {}", float32);
 	println!("I'm a floating-point 64-bit number {}", float64);

 	println!("Finally I am a character {}", two_hearts);
 }