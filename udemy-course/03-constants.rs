/*
	Constants in Rust
	-----------------
	Besides the library constants like:
		std::f64::consts::PI

	You have two types of constants in Rust:

		const MEANING_OF_LIFE: u8 = 42;

	Using the "const" keyword, what this does is at compile time, the compiler
	will substitute every appearance of MEANING_OF_LIFE with 42. The constant
	has no fixed address in memory. 

	But there are times that you need to have a constant with a memory address.
	For those cases you use the "static" keyword:

		static MY_NUMBER: u8 = 8;

	Always, bear in mind that in both cases you have to provide a type for
	constants.

	"static" can also be variables, if we include the "mut" keyword:
		static mut MY_LIFE: i32 = 10;

	However, the compiler will complain if we try to compile this directly.
	Why? Because declaring a static mut variable can compromise safety because
	of the "ownership" of this variable. Two processes can share the value and
	this is something Rust tries to avoid.
	The only way to use a static mut variable is in an "unsafe" code block.

		unsafe
		{
			println!("The value of my life is {}", MY_LIFE);
		}

	We can even change the value of the variable inside an unsafe block.
*/

const MEANING_OF_LIFE: u8 = 42;
static MY_NUMBER: u8 = 8;
static mut MY_LIFE: i32 = 10;

fn main() {
	println!("The value of Pi is {}", std::f64::consts::PI);
	println!("The meaning of life is {}", MEANING_OF_LIFE);
	println!("My secret number is {}", MY_NUMBER);

	unsafe
	{
		println!("The value of MY_LIFE is {}", MY_LIFE);
		MY_LIFE = 50;
		println!("The value of MY_LIFE is {}", MY_LIFE);
	}

}