/* Operators in Rust
 * Traditional arithmetic operators: + - / * 
 * No modulo operator, but % which is remainder
 * To increase or decrease by one ++ -- is not allowed.
 * But += -= /= *= %= is allowd.
 */

fn main() {

	let mut a: i32 = 2 + 3 * 4;
	println!("The value of a is {}", a);

	a = a + 1;			// no ++ or -- operators
	a -= 2;				// but += -= /= *= %= are allowed
	println!("The value of a is {}", a);

	println!("Remainder of {} / {} = {}", a, 3, (a % 3));

	// There is no power operator in Rust, but there is a function
	let a_cubed: i32 = i32::pow(a, 3);
	println!("{} cubed is {}", a, a_cubed);


	// check the differences between pow powi powf
	// pow for integers
	// powi for floats but the power is integral
	// powf for floats but both numbers are floats
	let b: f64 = 2.5;
	let b_cubed: f64 = f64::powi(b, 3);		// integer power
	let b_to_pi: f64 = f64::powf(b, std::f64::consts::PI);	// floating power

	println!("{} cubed {}", b, b_cubed);
	println!("{} to the power of {} is {}", b, std::f64::consts::PI, b_to_pi);

	// Bitwise operators
	// Are only available for integares
	let c: i32 = 1 | 2;		// or
							// & and
							// ^ xor
							// ! nor
	println!("1 | 2 = {}", c);

	// Shift operators
	let two_to_10: i32 = 1 << 10; // << shift to the left, >> shift to the right
	println!("2 ^ 10 = {}", two_to_10);

	// Logical operators
	// 
	let pi_less_4: bool = std::f64::consts::PI < 4.0;

	// The usual operators. < > <= >= == !=
	println!("Pi < 4.0 = {}", pi_less_4);
}


