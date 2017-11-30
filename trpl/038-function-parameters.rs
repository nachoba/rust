/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: FUNCTION PARAMETERS 
 * -----------------------------------------------------------------------------
 * Functions can also be defined to have parameters, which are special variables
 * that are part of a function's signature. When a function has parameters, we
 * can provide it with concrete values for those parameters.
 * Technically, the concrete values are called arguments, but in casual conver-
 * sation people tend to use the words "parameter" and "argument" interchangeably
 * for either the variables in a function's definition or the concrete values pas-
 * sed in when you call a function.
 */

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

/* The declaration of "another_function" has one parameter named "x". The type
 * of "x" is specified as "i32". 
 * In function signatures, you must declare the type of each parameter. This is
 * a deliberate decision in Rust's design; requiring type annotations in func-
 * tion definitions means the compiler almost never needs you to use them else-
 * where in the code to figure out what you mean.
 *
 * When you want a function to have multiple parameters, separate the parame-
 * ter declarations with commas:
 */

fn some_function(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}



fn main() {
    println!("Calling another_function(5) now....");
    another_function(5);

    println!("Calling some_function(5, 6) now....");
    some_function(5, 6);
}

/* Function Bodies
 * ---------------
 * Function bodies are made up of a series of statements optionally ending in
 * an expression. So far our functions don't have and ending expression, but we
 * have seen expressions as parts of statements. Because Rust is an expression
 * based language, this is an important distinction to understand.
 */


