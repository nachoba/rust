// A code block is also an expression and it may return a value.

fn main() {
	// first block
	let n1 = {
		let a = 2;
		let b = 5;
		a + b 
	};
	println!("The value of n1 is: {}", n1);

	// second block
	let n2 = {
		let a = 2;
		let b = 5;
		a + b;
	};
	println!("The value of n2 is: {:?}", n2);
}


/*

	The output is:

	The value of n1 is: 7
	The value of n2 is: ()


Note: code block {} end with a semicolon {};
Note: if you want the code block to return a value (other than the unit value
      "()"), you have to take out the semicolon in the last expression.

*/
