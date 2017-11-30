/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    ::  DIFFERENCES BETWEEN VARIABLES AND CONSTANTS
 * -----------------------------------------------------------------------------
 * Like immutable variables, constants are also values that are bound to a name
 * and are not allowed to change, but there are a few differences between them.
 *
 * 1. We are not allowed to use "mut" with constants. Constants are not immuta-
 *    ble by default, they are always immutable.
 * 2. We declare constants with the "const" keyword instead of "let", and the
 *    type of a constant must be always annotated.
 * 3. Constants can be declared in any scope, including the global scope, which
 *    makes them useful for values that many parts of code need to know about.
 * 4. Constants may only be set to a constant expression and not to the result
 *    of a function call or any other value that could be computed at runtime.
 *
 * Here we declare a constant in the global scope:
 */

const MAX_POINTS: u32 = 100_000;

/* Rust naming convention is to use all upper case with underscores between
 * words for constants.
 * Constants are valid for the entire time a program runs, within the scope they
 * were declared in, making a useful choice for values in you application domain
 * that multiple parts of the program might need to know about.
 *
 * Naming hardcoded values used throughout your program as constants is useful
 * in conveying the meaning of that value to future maintainers of the code. It
 * also helps to have only one place in your code you would need to change if
 * the hardcoded value needed to be updated in the future.
 */



fn main () {
    println!("The value of the constant MAX_POINTS is {}", MAX_POINTS);
}

