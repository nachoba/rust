/*
	The Stack and the Heap
	----------------------
	If you want to store a variable in the heap that would normally be stored 
	in the stack. You can use the Box::new function.

 */

use std::mem;

struct Point {
	x: f64,
	y: f64
}

fn origin() -> Point {
	Point {x: 0.0, y:0.0}
}


fn main() {
	let p1: Point = origin();
	println!("p1 is stored in the stack p1 = {} {}", p1.x, p1.y);
	let p2 = Box::new(origin());
	println!("p2 is stored in the heap p2  = {} {}", p2.x, p2.y);

	println!("p1 takes up {} bytes", mem::size_of_val(&p1));
	println!("That is p1 IS the actual value.");
	println!("p2 takes up {} bytes", mem::size_of_val(&p2));
	println!("That is p2 is a pointer to the heap. On a 64-bit system memory addresses occupy 8 bytes");

	// Now we want a p3 variable that holds the value of p2 but on the stack instead on the heap
	let p3: Point = *p2;		// We are unboxing the value.
	println!("p3 takes up {} bytes", mem::size_of_val(&p3));
	println!("p3 is stored in the stack p3 = {} {}", p3.x, p3.y);
}

