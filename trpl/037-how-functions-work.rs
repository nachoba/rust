/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: HOW FUNCTIONS WORK 
 * -----------------------------------------------------------------------------
 * Functions are pervasive in Rust code. The most important function in the lan-
 * guage is the "main" function, which is the entry point of many programs. The
 * "fn" keyword allows you to define new functions.
 *
 * Rust code uses snake case as the conventional style for function and variable
 * names. In snake case, all letters are lowercase and underscores separate the
 * words.
 */

fn another_function() {
    println!("This is printed from another function");
}


fn main() {
    println!("This is printed from main function");
    another_function();
}

/* Function definitions in Rust start with "fn" and have a set of parentheses
 * after the function name. The curly braces tell the compiler where the func-
 * tion body begins and ends.
 *
 * We can call any function we've defined by entering its name followed by a 
 * set of parentheses. Functions can be defined before or after the "main" func-
 * tion as long as they are defined somewhere.
 */

