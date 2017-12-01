/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: LOOPING THROUGH A COLLECTION WITH FOR 
 * -----------------------------------------------------------------------------
 * You could use the "while" construct to loop over the elements of a collection,
 * such as an array. For example:
 */

fn iterate_array() {

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value at {} is {}", index, a[index]);

        index = index + 1;
    }
}

/* Here, the code counts up through the elements in the array. It starts at
 * index 0, and then loops until it reaches the final index in the array.
 * But this approach is error prone; we could cause the program to panic if
 * the index length is incorrect. It's also slow, because the compiler adds
 * runtime code to perform the conditional check on every element on every
 * iteration through the loop.
 *
 * A more efficient alternative is to use a "for" loop and execute some code
 * for each item in a collection. A "for" loop looks like this:
 */

fn iterate_with_for() {
    let b = [11, 22, 33, 44, 55];

    for element in b.iter() {
        println!("the value is {}", element);
    }
}

/* We've now increased the safety of the code and eliminated the chance of bugs
 * that might result from going beyond the end of the array or not going far
 * enough and missing some items.
 * The safety and concisenes of "for" loops make them the most commonly used
 * loop construct in Rust. Even in situations in which you want to run some code
 * a certain number of times, most Rustaceans would use a "for" loop.
 * The way to do that would be to use a "Range", which is a type provided by the
 * standard ibrary that generates all numbers in sequences starting from one
 * number and ending before another number.
 *
 * Here is a countdown example using the "for" loop:
 */

fn countdown() {
    for number in (1..4).rev() {
        println!("The number is {}", number);
    }
}



fn main() {
    println!("Iterating over an array using while");
    iterate_array();

    println!("Iterating over an array using for");
    iterate_with_for();

    println!("Countdown example using for");
    countdown();
}


