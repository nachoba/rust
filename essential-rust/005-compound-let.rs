// Here is an exercise for you.
// Print out the values of:
// 							a
//							b
//							n
// After this code sinppet, and explain the value of "a"

fn main() {
	let mut a = 5;
	let mut b = 6;
	let n = 7;

	let a = b = n;	
	println!("a: {:?} b: {:?} n: {:?}", a, b, n);
}

/*

	a = 6
	b = 7
	n = 7 

	The correct answer is:
	a = ()
	b = 7
	n = 7
*/
