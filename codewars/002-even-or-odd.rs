/*	Even or Odd
	-----------
	Create a function that takes an integer as an argument
	and returns "Even" for even numbers or "Odd" for odd
	numbers.
 */


fn even_or_odd(i: i32) -> &'static str {
	match i % 2 {
		0 => "Even",
		_ => "Odd",
	}

}


fn main() {
	let num1: i32 = 0;
	let num2: i32 = 2;
	let num3: i32 = 1;
	let num4: i32 = 7;

	println!("The number {0} is {1}", num1, even_or_odd(num1));
	println!("The number {0} is {1}", num2, even_or_odd(num2));
	println!("The number {0} is {1}", num3, even_or_odd(num3));
	println!("The number {0} is {1}", num4, even_or_odd(num4));	
}