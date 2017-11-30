/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: SHADOWING 
 * -----------------------------------------------------------------------------
 * We can declare a new variable with the same name as a previous variable, and
 * the new variable shadows the previous one. In Rust jargon it is said that the
 * first variable is shadowed by the second, wich means that the second variable's
 * value is what we'll see when we use the variable.
 *
 * We can shadow a variable by using the same variabl's name and repeating the use
 * of the "let" keyword as follows:
 */


fn main() {
    
    let x = 5;

    println!("The value of x is         {}", x);

    let x = x + 1;

    println!("First shadowing x is now  {}", x);

    let x = x * 2;

    println!("Second shadowing x is now {}", x);



/* This is is different than marking a variable as "mut", because unless we use
 * the "let" keyword again, we will get a compile-time error if we accidentally
 * try to reassign to this variable. We can perform a few transformations on a 
 * value but have the variable be immutable after those transformations have
 * been completed.
 *
 * The other difference between "mut" and shadowing is that because we are ef-
 * fectively creating a new variable when we use the "let" keyword again, we
 * can change the type of the value, but reuse the same name.
 *
 * For example, say your program asks a user to show how many spaces they want
 * between some text by inputting space characters, but we really want to store
 * that input as number:
 */

    let spaces = "    ";
    println!("There are {} spaces here", spaces);
    let spaces = spaces.len();
    println!("There are {} spaces here", spaces);
}

/* This construct is allowd because the first "spaces" variable is a string type,
 * and the second "spaces" variable, which is a brand-new variable that happens
 * to have the same name as the first one, is a number type.
 * Shadowing spares us from having to come up with different names, like, let's
 * say "spaces_str" and "spaces_num". Instead we can use the simpler "spaces"
 * name. 
 * However, if we try to use "mut" for this we will get a compile-time error be-
 * cause we are not allowed to mutate a variable's type.
 */

