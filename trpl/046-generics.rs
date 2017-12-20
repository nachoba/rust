/*
 * THE RUST PROGRAMMING LANGUAGE
 * DEFINING AN USING GENERIC FUNCTIONS
 * -----------------------------------------------------------------------------
 * As Rust performs a strict data type check, when you define a function that
 * uses and argument of a certain type, the code that invokes such a function,
 * must pass to it expression of exactly that type, or it must perform explicit
 * conversions every time that function is used.
 */


 fn square_me(x: f32) -> f32 {
 	x * x
 }


/* This is inconvenient as Rust has many different numeric types, and perhaps
 * we want to pass the function a f64, u32, or i32.
 * The idiomatic way to solve this problem in Rust is to write the following
 * code:
 */

 fn square_me_generic<T>(x: T) -> T {
 	x * x
 }


 fn main() {
 	let x = 3.23_f64;
 	let z = 2.12_f32;
 	let y = 4_u32;
 	let n = 2;

 	println!("The square of {} is {}", x, square_me_generic(x));
 	println!("The square of {} is {}", z, square_me_generic(z));
 	println!("The square of {} is {}", y, square_me_generic(y));
 	println!("The square of {} is {}", n, square_me_generic(n));
 }
 