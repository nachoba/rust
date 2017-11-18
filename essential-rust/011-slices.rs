/* Slices in Rust
 * --------------
 * A 'slice' is a reference to (or "view" into) another data structure.
 * They are useful for allowing safe, efficient access to a portion of
 * an array without copying.
 * For example, you might want to reference only one line of a file
 * read into memory. By nature, a slice is not created directly, but
 * from an existing variable binding.
 * Slices have a defined length, and can be mutable or immutable.
 *
 * Internally, slices are repsented as a pointer to the beginning of the
 * data and a length.
 */






 /* Slicing Syntax
  * --------------
  * You can use a combo of & and [] to create a slice from various things.
  * The & indicates that slices are similar to references. The [], with a
  * range, let you define the length of the slice:
  */

fn main() {
	let a: [i32; 5] = [0, 1, 2, 3, 4];

  	let slice_complete = &a[..];		// a slice containing all the
  										// elements in `a`

  	let middle		   = &a[1..4];		// The slice of a containing
  										// only 1, 2, 3

  	println!("");									
  	print!("The slice slice_complete is: {:?}", slice_complete);

  	println!("");
  	println!("--------------------------------------------");

  	print!("The slice middle is:         {:?}", middle);
}

/* Slices have type &[T].
 */


