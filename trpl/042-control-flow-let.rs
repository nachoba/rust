/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: CONTROL FLOW USING IF IN A LET EXPRESSION 
 * -----------------------------------------------------------------------------
 * Because "if" is an expression, we can use it on the right side of a "let"
 * statement, for instance:
 */


fn main() {

    let condition: bool = true;
    let number: i8 = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);
}

/* The "number" variable will be bound to a value based on the outcome of the
 * "if" expression. Remember that blocks of code evaluate to the last expression
 * in them, and numbers by themselves are also expressions (they evaluate to them-
 * selves).
 * In this case, the value of the whole "if" expression depends on which block
 * of code executes. This means that the values that have the potential to be
 * results from each arm of the "if" must be the same type.
 * If types are mismatched and error would occur, stating that if and else have
 * incompatible types.
 * This is because Rust needs to know at compile time what type the "number" va-
 * riable is, definitively, so it can verify at compile time that its type is 
 * valid everywhere we use "number".
 *
 * Rust wouldn't be able to do that if the type of "number" was only determined
 * at runtime; the compiler wouls be more complex and would make fewer guarantees
 * about the code if it had to keep track of multiple hypothetical types for any
 * variable.
 */

