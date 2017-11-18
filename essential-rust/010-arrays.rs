/* Arrays in Rust
 * --------------
 * Rust has list types to represent a sequence of things. The most
 * basic is the "array", a fixed-size list of elements of the same
 * type. By default, arrays are immutable
 */


 fn main() {
 	let a: [i32; 3] = [1, 2, 3];	// a: [i32, 3]
 	let mut m = [2, 4, 6];			// m: [i32, 3]

 	// Arrays have type [T; N] where T is the type of the elements of
 	// the array and N is a compile-time constant, for the length of 
 	// the array.


 	// There's shorthand for initializing each element of an array to
 	// the same value:
 	let same_value: [i32; 20] = [0; 20];

 	// You can get the number of elements in an array with the
 	// .len() method
 	println!("a has {} elements", a.len());

 	// You can access a particular element of an array with a subscript
 	// notation:
 	let names: [&str; 3] = ["Batman", "Superman", "Thor"];
 	println!("The second superhero is: {}", names[1]);

 	// Subscript start at zero. If you try to use a subscript that is not
 	// in the array, you will get an error.
 }