/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: FUNCTIONS WITH RETURN VALUES 
 * -----------------------------------------------------------------------------
 * Functions can return values to the code that calls them. We don't name return
 * values, but we do declare their type after an arrow ( -> ). In Rust, the re-
 * turn value of a function is synonymous with the value of the final expression
 * in the block of the body of a function.
 *
 * You can return early from a function by using the "return" keyword and spe-
 * cifying a value, but most functions return the last expression implicitly.
 */

fn plus_one(x: i32) -> i32 {
    x + 1
}

/* Note that the function's return type is specified, too, as " -> i32"
 */

fn main() {
    let z: i32 = plus_one(4);
    println!("The value of z is: {}", z);
}

/* We are using the return value of a function to initialize a variable. Note
 * that in the "plus_one" function, the expression "x + 1" has no semicolon
 * because it's an expression whose value we want to return.
 *
 * If we place a semicolon at the end of the "x + 1" expression we will get an
 * error telling that there are mistmatched types. We said that the "plus_one"
 * function returns a value of type "i32", but statements don't evaluate to a
 * value, which is expressed by (), the empty tuple.
 *
 * Therefore, nothing is returned, which contradicts the function definition and
 * results in an error. 
 */




