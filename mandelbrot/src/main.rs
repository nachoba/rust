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
 * The preceding code defines a struct named "Complex", with two fields: "re" and 
 * "im". "Complex" is a generic structure: you can read the "<T>" after the type name
 * as "for any type T".
 * For example, "Complex<f64>" is a complex number whose "re" and "im" fields are
 * "f64" values, "Complex<f32>" would use 32-bit floats, and so on.
 * Given this definition, an expression like "Complex { re: R, im: I }" produces a
 * "Complex" value with its "re" field initialized to "R", and its "im" field initia-
 * lized to "I".
 *
 * The "num" crate arranges for *, +, and other arithmetic operators to work on Com-
 * plex values, so the rest of the function works just like the prior version, except
 * that it operates on points on the complex plane, not just points along the real
 * number line. We'll explain how you can make Rust's operators to work with your own
 * types, later.
 *
 * The Mandelbrot Set
 * ------------------
 * The set is defined as the set of complex numbers "c" for which "z" does not fly out
 * to inifity. Our original simple squaring loop was predictable enough: any number
 * greater than 1 or less than -1 flies away. Throwing a "a + c" into each iteration
 * makes the behavior a little harder to anticipate: as we said earlier, values of "c"
 * greater than 0.25 or less than -2 cause "z" to fly away. But expeanding the game to
 * complex numbers produces truly bizarre and beautiful patterns, which are what we
 * want to plot.
 *
 * Since a complex number "c" has both a real and imaginary components "c.re", and 
 * "c.im", we'll treat these as the "x" and "y" coordinates of a point on the Cartesian
 * plane, and color the point black if "c" is in the Mandelbro set, or a lighter color
 * otherwise.
 * So for each pixel in our image, we must run the preceding loop on the corresponding
 * point on the complex plane, see whether it escapes to infinity or orbits around the
 * origin forever, and color it accordingly.
 *
 * The inifinite loop takes a while to run, but there are two tricks for the impatient.
 * First, if we give up on running the loop forever and just try some limited number of
 * iterations, it turns out that we still get a decent approximation of the set.
 * How many iterations we need depends on how precisely we want to plot the boundary.
 * Second, it's been shown that, if "z" ever once leaves the circle of radius two cen-
 * tered at the origin, it will definitely fly infinitely far away from the origin 
 * eventually.
 *
 * So here's the final version of our loop, and the heart of our program:
 */

fn escape_time(c: Complex<f64>, limit: u32) -> Option<u32> {
    let muz z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }
    None
}

/* This function takes the complex number "c" that we want to test for membership in
 * the Mandelbrot set, and a limit on the number of iterations to try before giving
 * up and declaring "c" to probably be a member.
 *
 * The function's return value is on "Option<u32>". Rust's standard library defines
 * the "Option" type as follows:
 *
 *      enum Option<T> {
 *          None,
 *          Some(T),
 *      }
 *
 * "Option" is an enumerated type, often called an enum, because its definition enu-
 * merates several variants that a value of this type could be: for any type T, a value
 * Option<T> is either Some(v), where v is a value of type T; or None, indicating no
 * T value is available. 
 * Like the "Complex" type we discussed earlier, "Option" is a generic type: you can
 * use "Option<T>" to represent an optional value of any type T you like.
 *
 * In our case, "escape_time" returns an "Option<u32>" to indicate whether "c" is in 
 * the Mandelbrot set -and if it's not, how long we had to iterate to find that out.
 * If "c" is not in the set, "escape_time" returns "Some(i)", where "i" is the number
 * of the iteration at which "z" left the circle of radius two.
 *
 * Otherwise, "c" is apparently in the set, and "espcape_time" returns "None".
 *
 *      for i in 0..limit {
 *
 * This "for" loop simply iterates over the range of integers starting with 0 and up
 * to (but not including) "limit".
 *
 * The "z.norm_sqr()" method call returns the saqure of z's distance from the origin.
 * To decide whether "z" has left the circle of radius two, instead of computing a
 * square root, we just compare the squared distance with 4.0, which is faster.
 *
 * 
 * The /// comments lines above the function definition are documentation comments;
 * the "rustdoc" utility knows how to parse them, together with the code they describe,
 * and produce online documentation. The documentation for Rust's standard library is
 * written in this form.
 *
 * See here parsing.rs for Parsin Command Line Arguments
 */

fn main() {
    println!("Hello, world!");
}
