/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: CONDITIONAL LOOPS WITH WHILE 
 * -----------------------------------------------------------------------------
 * It's often useful for a program to evaluate a condition within a loop. While
 * the condition is true, the loop runs. When the condition ceases to be true,
 * you call "break", stopping the loop.
 *
 * This loop type could be implemented using a combination of "loop", "if", "else",
 * and "break". However this pattern is so common that Rust has a built-in language
 * contruct for it, and it's called a "while" loop.
 */

fn main() {
    let mut number = 30;

    while number != 0 {
        println!("The number is {}", number);

        number = number - 1;
    }

    println!("The loop ended!!");
}

/* This construct eliminates a lot of nesting that would be necessary if you used
 * "loop", "if", "else", and "break", and it's clearer. 
 * While the condition holds true, the code runs; otherwise, it exits the loop.
 */

