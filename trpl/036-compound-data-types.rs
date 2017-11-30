/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: COMPOUND DATA TYPES 
 * -----------------------------------------------------------------------------
 * Compound types can group multiple values of other types into one type. Rust
 * has two primitive compound types:
 *          * tuples
 *          * arrays
 *
 * Grouping Values into Tuples
 * ---------------------------
 * A tuple is a general way of grouping together some number of other values 
 * with a variety of types into one compound type.
 * We create a tuple by writing a comma-separated list of values inside parenthe-
 * ses. Each position in the tuple has a type, and the types of the different
 * values in the tuple don't have to be the same.
 */

fn main() {

    let tup: (i32, f64, u8)  = (500, 6.4, 1);

/* The variable "tup" binds to the entire tuple, since a tuple is considered a
 * single compound element. To get the individual values out of a tuple, we can
 * use pattern matching to destructure a tuple value:
 */

    let (x, y, z) = tup;
    println!("The value of y is: {}", y);

/* We use a pattern with "let" to take "tup" and turn it into three separate
 * variables, "x", "y", and "z". This is called destructuring, because it breaks
 * the single tuple into three parts.
 *
 * In addition to destructuring through pattern matching, we can also access a
 * tuple element directly by using a period ( . ), followed by the index of the
 * value we want to access:
 */

    let five_hundred    = tup.0;
    let six_point_four  = tup.1;
    let one             = tup.2;
    println!("The tuple contains these values: {} {} {}", 
             five_hundred, six_point_four, one);

/* We make three new variables for each element by using their index. As with
 * most programming languages, the first index in a tuple is 0.
 *
 *
 * Arrays
 * ------
 * Another way to have a collection of multiple values is with an array. Unlike
 * a tuple, every element of an array must have the same type. Arrays in Rust
 * have a fixed length: once declared, they cannot gow or shrink in size.
 *
 * The values going into an array are written as a comma-separated list inside
 * square brackets:
 */

    let a = [1, 2, 3, 4, 5];

/* Arrays are useful when you want your data allocated on the stack rather than
 * the heap, or when you want to ensure you always have a fixed number of ele-
 * ments.
 *
 * The vector type is a similar collection type provided by the standard libra-
 * ry that is allowed to grow and shrink in size; vectors are much flexible than
 * arrays.
 *
 * If you know that the number of items will not vary, such as the names of the
 * month in a year; then an array is the best solution.
 */

    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
/* Accessing Array Elements
 * ------------------------
 * An array is a single chunk of memory allocated on the stack. We can access
 * elements of an array using indexing:
 */

    let current_month = months[10];
    println!("We are in {} now", current_month);

/* Suppose we evaluate:
 *
 *  let current_month = months[14];
 *
 * The code will compile, but the program will result in a runtime error and will
 * not exit successfully. When you attempt to access an element using indexing,
 * Rust will check that the index you've specified is less than the array length.
 * If the index is greater than the length, Rust will "panic", which is the term
 * Rust uses when a program exits with an error.
 *
 * This is an example of Rust's safety principles in action. In many low-level
 * programming languages, this kind of check is not done, and when you provide an
 * incorrect index, invalid memory can be accessed. Rust protects you against this
 * kind of error by immediately exiting instead of allowing the memory access and
 * continuing.
 */

}

