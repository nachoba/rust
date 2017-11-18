/*
 * Functions in Rust
 */



 fn add_numbers(x: i32, y: i32) -> i32 {
 	x + y		// no semicolon here, we want the function to return 
 				// a value and not the empty tuple ().
 }

 fn print_number(x: i32) {
 	println!("The number is {}", x);
 }


fn plus_one(x: i32) -> i32 {
	x + 1
}




fn main() {
	let num1: i32 = 256;
	let num2: i32 = 256;

	let result: i32 = {
		let num1: i32 = 32;
		let num2: i32 = 32;
		add_numbers(num1, num2)		// no semicolon here we want the 
									// code block to return this
	};

	// create a pointer to the plus_one function
	let p1: fn(i32) -> i32 = plus_one;
	let six: i32 = p1(5);

	print_number(add_numbers(num1, num2));
	print_number(result);
	println!("Using p1 (pointer to plus_one) we get {}", six);
	println!("Using plus_one directly we get {}", plus_one(5));
}

