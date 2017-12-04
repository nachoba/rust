/* PROGRAMMING RUST
 * -----------------------------------------------------------------------------
 * Chapter 3 :: Basic Types
 * ---------
 * Rust's types serve several goals:
 *
 * Safety       By checking a program's types, the Rust compiler rules out whole
 *              classes of common mistakes. By replacing null pointers and un-
 *              checked unions with type-safe alternatives, Rust is even able to
 *              eliminate errors that are common sources of crashes in other lan-
 *              guages.
 *
 * Efficiency   Programmers have fine-grained controil over how Rust programs re-
 *              present values in memory, and can choose types they know the pro-
 *              cessor will handle efficiently. Programs needn't pay for generality
 *              or flexibility they don't use.
 *
 * Concision    Rust manages all of this without requiring too much guidance from
 *              the programmer in the form of types written out in the code. Rust
 *              programs are usually less cluttered with types than the analogous
 *              C or C++ program should be.
 *
 * Rust is designed to use ahead-of-time compilation: the translation of your en-
 * tire program to machine code is completed before it ever begins execution.
 * Rust's types help an ahead-of-time compiler choose good machine-level repre-
 * sentations for the values your program operates on: representations whose per-
 * formance you can predict, and which give you full access to the machine's ca-
 * pabilities.
 *
 * Rust is a statically typed language: without actually running the program, the
 * compiler checks that every possible path of execution will use values only in
 * ways consistent with their types. This allows Rust to catch many programming
 * mistakes early, and is crucial to Rust's safety guarantees.
 *
 * Compared to a dynamically typed language like JavaScript or Python, Rust requires
 * more planning from you up front: you must spell out the types of functions' para-
 * meters and return values, members of struct types, and a few other constructs.
 * However, two features of Rust make this less trouble than you might expect:
 *
 *      * Given the types that you did spell out, Rust will infer most of the rest
 *        for you. In practice, there's often only one type that will work for a
 *        given variable or expression; when this is the case, Rust lets you leave
 *        out the type. For example, you could spell out every type in a function,
 *        like this:
 *
 *        fn build_vector() -> Vec<i16> {
 *              let mut v: Vec<i16> = Vec::<i16>::new();
 *              v.push(10i16);
 *              v.push(20i16);
 *              v
 *        }
 *
 *        But this is cluttered and repetitive. Given the function's return type, it's
 *        obvious that v must be a Vec<i16>, a vector of 16-bit integers; no other
 *        type would work. And from that it follows that each element of the vector
 *        must be an i16. This is the sort of reasoning that Rust's type inference
 *        applies, allowing you to instead write:
 *
 *        fn build_vector() -> Vec<i16> {
 *              let mut v = Vec::new();
 *              v.push(10);
 *              v.push(20);
 *              v
 *        }
 *
 *        Rust will generate the same machine code either way. Type inference gives
 *        back much of the legibility of dynamically typed languages, while still
 *        catching type errors at compile time.
 *
 *      * Functions can be generic: when a function's purpose and implementation are
 *        general enough, you can define it to work on any set of types that meet
 *        the necessary criteria. A single definition can cover an open-ended set of
 *        use cases. Rust's generic functions give the language a great degree of
 *        flexibility, while still catching all type errors at compile time.
 *        Despite their flexibility, generic functions are just as efficient as their
 *        nongeneric conterparts.
 *
 * Machine Types
 * -------------
 * The base of Rust's type system is a collection of fixed-width numeric types, chosen
 * to match the types that almost all modern processors implement directly in hardware,
 * and the Boolean and character types.
 *
 * The names of Rust's numeric types follow a regluar pattern, spelling out their width
 * in bits, and the representation they use:
 *
 *      Size            Unsigned Integer        Signed Integer      Floating-Point
 *      --------------------------------------------------------------------------
 *      8                   u8                      i8
 *      16                  u16                     i16
 *      32                  u32                     i32                 f32
 *      64                  u64                     i64                 f64
 *      Machine word        usize                   isize
 *      --------------------------------------------------------------------------
 *
 *  Here, a machine word is a value the size of an address on the machine the code runs
 *  on, usually 32-bit or 64-bit.
 *
 *
 *
 * Integer Types
 * -------------
 *
