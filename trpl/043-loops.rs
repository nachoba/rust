/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: REPETITION WITH LOOPS 
 * -----------------------------------------------------------------------------
 * To execute a block of code more than once, Rust provides several loops. A loop
 * runs through the code inside the loop body to the end and then starts immedia-
 * tely back at the beginning.
 *
 * Repeating Code with "loop"
 * --------------------------
 * The "loop" keyword tells Rust to execute a block of code over and over again
 * forever or until you explicitly tell it to stop. For example:
 */

fn main() {
    loop {
        println!("again");
    }
}

/* When you run this program, we'll see "again" printed over and over continuosly
 * until we stop the loop manually (with CTRL-c). Fortunatelly, Rust provides a-
 * nother, more reliable way to break out of a loop. You can place the "break"
 * keyword withing the loop to tell the program when to stop executing the loop.
 */

