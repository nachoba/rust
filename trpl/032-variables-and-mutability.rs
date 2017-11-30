/*
 *  THE RUST PROGRAMMING LANGUAGE
 *  CHAPTER 3   ::  VARIABLES AND MUTABILITY
 *  ----------------------------------------------------------------------------
 *  All variables are, by default immutable. This is one of the way Rust encou-
 *  rages you to write your code in a way that takes advance of the safety and 
 *  easy concurrency that Rust offers.
 *  However, you still have the option to make your variables mutable. When a
 *  variable is immutable, that means that once a value is bound to a name, you
 *  can't change that value. For example:
 */

fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    println!("x is immutable. If we try to do this:");
    println!("x = 6;");
    println!("We would get an error:");
    println!("re-assignment of immutable variable `x`");
    println!("");

/* This example shows how the compiler helps you find errors in your programs.
 *
 * It's important that we get compile-time errors when we attempt to change a
 * value that we previously designated as immutable because this very situation
 * can lead to bugs. If one part of our code operates on the assumption that a
 * value will never change and another part of our code changes that value, it's
 * possible that the first part of the code won't do what is was designed to do.
 * This cause of bugs can be difficult to track down after the fact, especially
 * when the second piece of code changes the value only sometimes.
 *
 * In Rust the compiler guarantees that when we state that a value won't change,
 * it really won't change. But mutability can be very useful. Variables are im-
 * mutable only by default; we can make them mutable by adding "mut" in front of
 * the variable name. 
 *
 * In addition to allowing this value to change, it conveys intent to future rea-
 * ders of the ode by indicating that other parts of the code will be changing
 * this variable value.
 */

    let mut z = 5;
    println!("Now z is defined as a mutable variable with the 'mut' keyword");
    println!("The value of z is now     : {}", z);
    z = 10;
    println!("The value of z changes to : {}", z);
}

/* Using "mut", we're allowed to change the value that `z` binds to from 5 to 10.
 * There are multiple trade-offs to consider, in addition to prevention of bugs.
 * For example, in cases where you're using large data structures, mutating an
 * instance in place may be faster than copying and returning newly allocated ins-
 * tances. 
 * With smaller data structures, creating a new instance and writing in a more 
 * functional programming style may be easier to reason about, so the lower
 * performance might be a worthwhile penalty for gaining in clarity.
 */

