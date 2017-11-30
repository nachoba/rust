/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: STATEMENTS AND EXPRESSIONS 
 * -----------------------------------------------------------------------------
 * Statements are instructions that perform some action and do not return a value.
 * Expressions evaluate to a resulting value. Let's look at some examples:
 *
 * Creating a variable and assigning a value to it with the "let" keyword is a
 * statement.
 *
 *      let y: i16 = 6;                 // this is a statement
 *
 * Function definitions are also statements. Statements do not return values.
 * Therefore, you can't assign a "let" statement to another variable, as the
 * following code tries to do:
 *
 *      let x = (let y = 6);    // error: expected expression, found statement
 *
 * The (let y = 6) statement does not return a value, so there isn't anything
 * for "x" to bind to. This is different than in other languages, such as C and
 * Ruby, where the assignement returns the value of the assignment. In those
 * languages you can write "x = y = 6" and have both "x" and "y" have the value
 * 6; that is not the case in Rust.
 *
 * Expressions evaluate to something and make up most of the rest of the code
 * that you'll write in Rust. Consider a simple math operation, such as 5 + 6,
 * which is an expression that evaluates to the value 11.
 * Expressions can be part of statements, for instance in the statement:
 *
 *      let y = 6
 *
 * 6 is an expression that evaluates to 6. Calling a function is an expression.
 * Calling a macro is an expression. Blocks are also expressions:
 */


fn main() {

    let x: i8 = 5;

    let y: i8 = {
        let x: i8 = 3;          // shadowing of x
        x + 1
    };

    println!("The value of y is: {}", y);
}

/* The block evaluates to 4. That value gets bound to "y" as part of the "let"
 * statement. Note the "x + 1" line without a semicolon at the end, unlike most
 * of the lines you've seen so far.
 * Expression do not include ending semicolons. If you add a semicolon to the
 * end of an expression, you turn it into a statement, which will then not re-
 * turn a value. Keep this in mind as you explore functions with return values.
 */

