/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: CONTROL FLOW 
 * -----------------------------------------------------------------------------
 * The most common constructs that let you control the flow of execution of Rust
 * code are "if" expressions and loops.
 *
 * "if" Expressions
 * ----------------
 * An "if" expression allows us to branch our code depending on conditions. We
 * provide a condition and then state, "if this condition is met, run this block
 * of code. If the condition is not met, do not run this block of code".
 *
 * All "if" expressions start with the keyword "if", which is followed by a con-
 * dition. In this case, the condition checks whether or not the variable "number"
 * has a value less than 5.
 */

fn main() {
    let number: i8 = 3;

    if number < 5 {
        println!("the condition was true");
    } else {
        println!("the condition was false");
    }


/* Blocks of code associated with the conditions in "if" expressions are some-
 * times called arms, just like the arms in "match" expressions. Optionally, we
 * can also include an "else" expression, which we chose to do here, to give the
 * program an alternative block of code to execute should the condition evaluate
 * to false.
 *
 * If you don't provide an "else" expression and the condition is false, the pro-
 * gram will just skip the "if" block and move on to the next bit of code.
 *
 * The condition must be a bool, that is, you must be explicit and always provide
 * and "if" with a boolean as its condition.
 *
 * Multiple Conditions with "else if"
 * ----------------------------------
 * We can have multiple conditions by combining "if" and "else" in an "else if"
 * expression:
 */

    let number: i8 = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}

/* Using too many "else if" expressions can clutter your code, so if you have
 * more than one, you might want to refactor your code, perhaps using another
 * branching construct such as "match".
 */


