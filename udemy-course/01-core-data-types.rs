/*
	Core Data Types
	===============

 */

use std::mem;

fn main() {
	let a: u8 = 123;		// 8-bits 

	println!("a is an unsigned 8-bit integer = {}", a);


	// bindings are immutable by default. Use mut to make it mutable
	let mut b: i8 = 0;		// mutable
	println!("b is a signed 8-bit integer and it's mutable = {}", b);
	b = 42;
	println!("Now b has a value of = {}", b);


	// Rust has type inference
	let mut c = 123456789;	
	println!("Type inference: c = {}, size = {} bytes", c, mem::size_of_val(&c));
	c = -1;
	println!("Check that it's a signed 32-bit integer c = {}, size = {} bytes", c, mem::size_of_val(&c));


	// isize usize 
	// integer data types that correspond to the size of a memory address
	// location, that is on:
	// 32-bit OS = i32 = 4 bytes
	// 64-bit OS = i64 = 8 bytes
	let z: isize = 123;
	let size_of_z = mem::size_of_val(&z);
	println!("isize z = {}, takes up {} bytes, {}-bit os",
		z, size_of_z, size_of_z * 8);

	// chars
	let d: char = 'x';
	println!("d is a char with a value = {}, size = {} bytes", d, mem::size_of_val(&d));
	// chars have a size of 4-bits full UNICODE UTF-8


	// Floats
	let e = 2.5;		// defult f64
	println!("Type inference: e = {}, size = {} bytes", e, mem::size_of_val(&e));

	let f: f32 = 3.14;
	println!("Type inference: f = {}, size = {} bytes", f, mem::size_of_val(&f));


	// Booleans

	let g = false;
	let h: bool = true;
	println!("Type inference: g = {}, size = {} bytes", g, mem::size_of_val(&g));
	println!("h = {}, size = {} bytes", h, mem::size_of_val(&h));


}