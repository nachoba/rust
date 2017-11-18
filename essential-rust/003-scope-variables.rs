// Scope of a variable and shadowing
fn main() {
	let outer = 42;

	{	// start of code block
		let inner = 3.14;
		println!("block variable inner = {}", inner);
		let outer = 99;
		println!("block variable outer = {} (shadowing!)", outer);
	}	// end of code block

	println!("value of outer (outside the code block) = {}", outer);
}
