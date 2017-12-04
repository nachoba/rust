/* Mandelbrot Set Using Concurrency
 * --------------------------------
 * From: Learning Rust
 * Date: 03/12/17
 *
 * In this section, we'll walk through the process of writting a multi-threated
 * program. This program plots the Mandelbrot set, a fractal produced by itera-
 * ting a simple function on complex numbers. Plotting the Mandelbrot set is
 * often called an embarrassingly parallel algorithm, because the pattern of
 * communication between the threads is so simple.
 *
 * What the Mandelbrot Set Actually Is
 * -----------------------------------
 * We'll start with a somple case, and then add complicating details until we
 * arrive at the calculation at the heart of the Mandelbrot set.
 * Here's an infinite loop, written using Rust's dedicated syntax for that, a
 * "loop" statement:
 *
 *		fn square_loop(mut x: f64) {
 *			loop {
 *				x = x * x;
 *			}
 *		}
 *
 * What happens to the value of x?
 * Squaring any number smaller than 1 makes it smaller, so it approaches zero;
 * squaring 1 yields 1; squaring a number larger than 1 makes it larger, so it
 * approaches infinity; and squaring a negative number makes it positive, after
 * which it behaves as one of the prior cases.
 *
 * So, depending on the value you pass to "square_loop", x either approaches zero,
 * stays at 1, or approaches infinity.
 *
 * Now let's consider a slightly different loop:
 *
 *		fn square_add_loop(c: f64) {
 *			let mut x = 0;
 *			loop {
 *				x = x * x + c;
 *			}
 *		}
 *
 * This time, "x" starts at zero, and we tweak its progress in each iteration by
 * adding in "c" after squaring it. This makes it harder to see how "x" fares, but
 * some experimentation shows that if "c" is greater than 0.25, or less than -2.0,
 * then "x" eventually becomes infinitely large; otherwise, it stays somewhere in
 * the nieghborhood of zero.
 *
 * Next, instead of using "f64" values, consider the same loop using complex num-
 * bers. The "num" crate on "crates.io" provides a complex number type we can use,
 * so we mus add a line for "num" to the "[dependencies]" section of our program's
 * Cargo.toml file. We add:
 *
 *		....
 *		[dependencies]
 *		num = "0.1.27"
 *
 * Also we must add an "extern crate" and a "use" statement to allow our program
 * to use this crate:
 */

extern crate num;
use num::Complex;

#[allow(dead_code)]
fn complex_square_add_loop(c: Complex<f64>) {
	let mut z = Complex { re: 0.0, im: 0.0};
	loop {
		z = z * z + c;
	}
}

/* It's traditional to use "z" for complex numbers, so we've renamed our looping
 * variable. The expression "Complex { re: 0.0, im: 0.0}" is the way we write
 * complex zero using the "num" crate's "Complex" type. "Complex" is Rust struc-
 * ture type (or struct), defined like this:
 *
 *		struct Complex<T> {
 *			/// Real portion of the complex number
 *			re: T,
 *
 *			/// Imaginary portion of the complex number
 *			im: T
 *		}
 *
 *
 */
 



fn main() {
    println!("Hello, world!");
}
