/* Tuples
   ------

   A tuple is an ordered list of fixed size, like this:

   let x = (1, "hello");

   The parentheses and commas form this two-length tuple.
   Here is the same expression but with the type annotated:

   let x: (i32, &str) = (1, "hello");

   As you can see, the type of a tuple looks like the tuple,
   but with each position having a type name rather than a
   value. Tuples are heterogenous, they can have elements of
   different types.

   You can assign one tuple into another, if they have the same
   contained types and arity. Tuples have the same arity when
   they have the same length
 */

fn main() {

	let x: (i32, &str) = (12, "hello");

	let mut z: (i32, &str) = x;

	println!("The tuple x is {:?}", x);
	println!("The tuple z is {:?}", z);


/* You can access the fields in a tuple through a destructing let.
   Here's an example:
 */

 	let (a, b) = x;
 	println!("the first element of the x tuple is {:?}", a);

/* Here you see how the "let" binding is more powerful than assigning.
   We can put a pattern on the left-hand side of the "let", and if it
   matches up to the right-hand side, we can assign multiple bindings 
   at once. In this case, the "let" destructures or breaks-up the tuple,
   and assigns the bits to two bindings.

   You can disambiguate a single-element tuple from a value in parentheses
   with a comma:

   (0,);	// a single-element tuple.
   (0);		// a zero in parentheses.



   Tuple Indexing
   --------------
   You can also access fields of a tuple with indexing syntax:
 */

 	let tuple: (i32, i32, i32) = (1, 2, 3);

 	let x1: i32 = tuple.0;
 	let x2: i32 = tuple.1;

 	println!("The value of x1 is {} and of x2 is {}", x1, x2);
 	println!("The value of x3 is {}", tuple.2); 

