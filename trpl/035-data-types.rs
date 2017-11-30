/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: DATA TYPES 
 * -----------------------------------------------------------------------------
 * Every value in Rust is of a certain type, which tells Rust what kind of data
 * is being specified so it knows how to work with that data. For a better un-
 * derstanding of types we will split them into two subsets:
 *          * scalar
 *          * compound
 *
 * Rust is a statically typed language, which means that it must know the types
 * of all variables at compile time. The compiler can usually infer what type we
 * want to use based on the value and how we use it.
 * In cases when many types are possible, such as when we converted a "String"
 * to a numeric type using "parse", we must use type annotations, like this:
 *
 *      let guess: u32 = "42".parse().expect("Not a number!");
 *
 *  If we don't add the type annotation here, Rust will display the following
 *  error, meaning the compiler needs more information from us to know which
 *  possible type we want to use:
 *
 *      error: type annotation needed
 *
 *  SCALAR TYPES
 *  ------------
 *  A scalar type represents a single value. Rust has four primary scalar types:
 *          * integers
 *          * floating-point
 *          * booleans
 *          * characters
 *
 *  Integers
 *  --------
 *  An integer is a number without a fractional component. Signed integers types
 *  start with an "i", unsigned with an "u". The following table shows the built
 *  in integer types:
 *
 *          Length      Signed      Unsigned
 *          ------      ------      --------
 *          8-bit       i8          u8
 *          16-bit      i16         u16
 *          32-bit      i32         u32
 *          64-bit      i64         u64
 *          arch        isize       usize
 *          ------      ------      --------
 *
 * Each variant can be either signed or unsigned and has an explicit size. The
 * "isize" and "usize" types depend on the kind of computer your program is run-
 * ning on: 64-bit if you are on a 64-bit architecture and 32-bits if you are in
 * a 32-bit architecture.
 *
 * You can write integer literals in any of the following forms:
 *
 *          Number Literals         Examples
 *          ---------------         -----------
 *          Decimal                 98_222
 *          Hexadecimal             0xff
 *          Octal                   0o77
 *          Binary                  0b1111_0000
 *          Byte (u8 only)          b'A'
 *          ---------------         -----------
 *
 *  Note that all number literals except the byte literal allow a type suffix,
 *  such as 57u8, or 1_234_452u32. And an underscore _ as visual separator.
 *
 *          How do you know which type of integer to use? If you're unsure
 *          Rust's defaults are generally good choices, and integer types
 *          default to i32. It's generally the fastest, even on 64-bit sys-
 *          tems.
 *          The primary situation in which you'd use "isize" or "usize" is
 *          when indexing some sort of collection.
 *
 *  Floating-Point
 *  --------------
 *  Rust has two primitive for floating-point numbers, which are numbers with
 *  decimal points. Rust's floating-point types are "f32" and "f64". The de-
 *  fault type is "f64" because on modern CPUs it's roughly the same speed as
 *  "f32" but is capable of more precision.
 */

 fn main() {

     let x      = 2.0;          // by default Rust assumes "f64"
     let y: f32 = 3.0;          // explicitly state that we need a "f32"



/* Numeric Operations
 * ------------------
 * Rust supports the usual basic mathematical operations you'd expect for all
 * the number types: addition, subtraction, multiplication, division, and re-
 * mainder.
 */

     let sum        = 5 + 10;           // addition

     let difference = 95.5 - 4.3;       // subtraction

     let product    = 4 * 30;           // multiplication

     let quotient   = 56.7 / 32.2;      // division

     let remainder  = 43 % 5;           // remainder

/* Each expression in these statements uses a mathematical operator and evaluates
 * to a single value, which is then bound to a variable.
 *
 * Booleans
 * --------
 * As in other programming languages, a boolean type in Rust has two possible
 * values: "true" and "false". The boolean type in Rust is specified using the
 * "bool" keyword:
 */

     let t       = true;                 // t has type bool
     let f: bool = false;                // f has type bool

/* The main way to consume boolean values is through conditionals, such as an
 * "if" expression.
 *
 * Characters
 * ----------
 * Rust support letters too. Rust's "char" type is the language's most primitive
 * alphabetic type. Note that the "char" type is specified with single quotes,
 * as opposed to strings that use double quotes.
 */

     let c            = 'z';                  // c has type char
     let emoji: char  = '😻';                 // emoji has type char

     println!("c has type char and the value is    {}", c);
     println!("emoji is also char and the value is {}", emoji);

/* Rust's "char" type represents a Unicode Scalar Value, which means it can re-
 * present much more character than ASCII. In Rust unicode scalar values range
 * from U+0000 to U+D7FF and U+E000 to U+10FFFF inclusive.
 */

}

