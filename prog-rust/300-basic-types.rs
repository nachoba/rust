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
 * Rust's unsigned integer types use their full range to represent positive values and
 * zero. Rust's signed integer types use the two's complement representation, using
 * the same bit patterns as the corresponding type to cover a range of positive and
 * negative values.
 *
 * Rust generally uses the "u8" type for byte values; reading data from a file or 
 * socket yields a stream of "u8" values.
 * Rust treats characters as distinct from the numeric types; a "char" is neither a
 * "u8" nor an "i8".
 * The "usize" and "isize" types are machine-dependent. Their precision depends on 
 * the size of the address space on the target machine: they are 32-bit long on
 * 32-bit architectures, and 64-bit long on 64-bit architectures.
 * Rust requires array indices to be "usize" values. Values representing the sizes
 * of arrays or vectors or counts of the number of elements in some data structure
 * also generally have the size "usize" type.
 *
 * Integer literals in Rust can take a suffix indicating their type: "42u8" is a "u8"
 * value, and "1729isize" is an "isize". You can omit the suffix on an integer literal,
 * in which case Rust will try to infer a type for it from the context. If on doubt,
 * Rust defaults to "i32".
 *
 * The prefixes 0x 0o 0b designate hexadecimal, octal, and binary literals.
 * To make long numbers more legible, you can insert underscores among the digits.
 * For example, you can write the largest "u32" value as 4_294_967_295.
 *
 * Although numeric types and the "char" type are distinct, Rust does provide byte
 * literals, character-like literals for "u8" values: b'X' represents the ASCII
 * code for the character X, as an "u8" value.
 * Since the ASCII code for A is 65, the literals b'A' and 65u8 are exactly equivalent.
 * Only ASCII characters may appear in byte literals and there're few characters that
 * you cannot simply place after the single quote, and you have to use a backslash:
 *
 *      Character           Byte Literal        Numeric Equivalent
 *      ----------------------------------------------------------
 *      Single quote '      b'\''               39u8
 *      Backslash    \      b'\\'               92u8
 *      Newline             b'\n'               10u8
 *      Carriage Return     b'\r'               13u8
 *      Tab                 b'\t'               9u8
 *      ----------------------------------------------------------
 *
 *
 * You can convert from one integer type to another using the "as" operator. Here are
 * some examples:
 *
 *      assert_eq!(   10_i8  as u16,   10_i16);  // in range
 *      assert_eq!( 2525_u16 as i16, 2525_i16);  // in range
 *
 * Conversions that are out of range for the destination, produce values that are
 * equivalent to the original module 2^n, where n is the width of the destination in
 * bits. This is sometimes called "truncation":
 *
 *      assert_eq!( 255_u8  as i8,  -1_i8);
 *      assert_eq!(  -1_i8  as u8, 255_u8);
 *
 * Like any other sort of value, integers can have methods, the standard library pro-
 * vides some basic operations.
 *
 *
 * Floating-Point Types
 * --------------------
 * Rust provides single and double precision floating-point types. These types
 * include positive and negative infinities, distinct positive and negative zero
 * values, and a not-a-number (NAN) value.
 * Rust's "f32" and "f64" have the following general form:
 *
 *      integer_part fractional_part exponent type_suffix
 *      31415.926e-4f64
 *
 * Rust defaults to "f64" if both "f32" and "f64" are possible during type inference.
 *
 * The standard library's "std::f32" and "std::f64" modules define constants for special
 * values like INFINITY, NEG_INFINITY, NAN, MIN, MAX
 * The stadard library's "std::f32::consts" and "std::f64::consts" modules provide 
 * various commonly used mathematical constants like E, PI, and the square root of two.
 *
 * "f32" and "f64" types provide a full complement of methods for mathematical calcu-
 * lations; for example:
 *
 *      2f64.sqrt();
 *      1.0f32.sqrt();
 *      -1.1f64.floor();
 *      -1f32.is_sign_negative();
 *
 * Rust performs almost no numeric conversions implicitly; if a function expects an
 * "f64" argument, it's an error to pass an "i32" value as the argument. In fact, Rust
 * won't even implicitly convert an "i16" values to an "i32" value, even though every
 * "i16" value is also an "i32" value. But you can always  write out explicit conver-
 * sions using the "as" operator.
 *
 *
 * The bool Type
 * -------------
 * Rust's Boolean type "bool", has the usual two values for such types: "true" and
 * "false". Comparison operators like == and < produce "bool" results: the value of
 * 2 < 5 is "true".
 * Rust is very strict: control structures like "if" and "while" require their condi-
 * tions to be "bool" expressions, as do the short-circuiting logical operators && 
 * and ||. You must write:
 *		
 *		if x != 0 {
 *			...
 *		}
 *
 * and not:
 *
 *		if x {
 *			...
 *		}
 *
 * Rust's "as" operator can convert "bool" values to integer types:
 *
 *		assert_eq!(false as i32, 0);
 *		assert_eq!(true  as i32, 1);
 *
 * However, "as" won't convert in the other direction, from numeric types to "bool".
 * So you must write out an explicit comparison like:
 *
 *		x != 0
 *
 * Although a "bool" only needs a single bit to represent it, Rust uses an entire
 * byte for a "bool" value in memory, so you can create a pointer to it.
 *
 * Characters
 * ----------
 * Rust's character type "char" represents a single Unicode character, as a 32-bit
 * value. Rust uses the "char" type for single characters in isolation, but uses
 * the UTF-8 encoding for strings and streams of text. So, a "String" represents
 * its text as a sequence of UTF-8 bytes, not as an array of characters.
 *
 * Character literals are enclosed in single quotes, like '8' or '!'. You can use
 * any Unicode character you like. As with byte literals, backslash escapes are
 * required for a few characters.
 *
 * If you prefer, you can write out a character's Unicode code point in hexadeci-
 * mal:
 *
 *		* If the character's code point is in the range U+0000 to U+007F, then
 *		  you can write the characters as '\xHH'. For example, the character
 *		  literals '*' and '\x2A' are equivalent, because the code point of the
 *		  character * is 42, or 2A in hexadecimal.
 *		* You can write any Unicode character as '\u{HHHHHH}', where HHHHHH is a
 *		  hexadecimal number between one and six digits long. For example, the
 *		  character literal '\u{CA0}' represents the character, Kannada used in 
 *		  the Unicode Look of Disapproval.
 *
 * A "char" always holds a Unicode point in the range 0x0000 to 0xD7FF, or
 * 0xE000 to 0x10FFFF. A "char" is never a surrogate pair half (a point in the
 * range 0xD800 to 0xDFFF), or a value outside the Unicode codespace (greater
 * than 0x10FFFF. Rust uses the type system and dynamic checks to ensure "char"
 * values are always in the permitted range.
 * You can use the "as" operator to convert a "char" to an integer type; for
 * types smaller than 32 bits, the upper bits of the character's value are trun-
 * cated.
 * Going in the other direction, "u8" is the only type the "as" operator will
 * convert to "char". The standard library provides some useful methods on cha-
 * racter, which you can look up in the online documentation by searching for
 * "char (primitive type)", and for the module "std::char"
 *
 * Tuples
 * ------
 * A tuple is a pair, or triple, or quadruple, ... of values of assorted types. You
 * can write a tuple as a sequence of elements, separated by commas and surrounded
 * by parentheses. For example:
 *
 *      ("Brazil", 1985)
 *
 * is a tuple whose first element is a statically allocated string, and whose second
 * is an integer; its type is "(&str, i32)". Given a tuple value "t", you can access
 * its elements as t.0, t.1, and so on.
 *
 * Tuples are different than arrays:
 * 1. Each element of a tuple can have a different type, whereas an array's elements
 *    must be all the same type.
 * 2. Tuples allow only constants as indices, like t.4. You can't write t.i or t[i]
 *    to get the i'th element.
 *
 * Rust code often uses tuple types to return multiple values from a function. For
 * example, the "split_at" method on string slices, which divides a string into two
 * halves and returns them both, is declared like this:
 *
 *      fn split_at(&self, mid: usize) -> (&str, &str);
 *
 * The return type "(&str, &str)" is a tuple of two string slices. You can use pat-
 * tern matching syntax to assign each element of the return value to a different
 * variable:
 *
 *      let text = "I see the eigenvalue in thine eye";
 *      let (head, tail) = text.split_at(21);
 *      assert_eq!(head, "I see the eigenvalue ");
 *      assert_eq!(tail, "in thine eye");
 *
 * One of the most commonly used tuples is the zero-tuple (). This is traditionally
 * called the unit type because it has only one value, also written (). Rust uses the
 * unit type where there's no meaningful value to carry, but context requires some 
 * sort of type nonetheless. For example, a function that returns no value has a
 * return type of (). The standard library's "std::mem::swap" function has no mea-
 * ningful return value; it just exchanges the value of its two arguments. The de-
 * claration for "std::mem::swap" reads:
 *
 *      fn swap<T>(x: &mut T, y: &mut T);
 *
 * The "<T>" means that swap is generic: you can use it on references to values of
 * any type "T". But the signature omits the swap's return type altogether, which is
 * shorthand for returning the unit type:
 *
 *      fn swap<T>(x: &mut T, y: &mut T) -> ();
 *
 * If you like, you may include a comma after a tuple's last element: the types
 * "(&str, i32,)" and "(&str, i32)" are equivalent, as are the expressions
 * "("Brazil", 1985, )" and "("Brazil", 1985)".
 * Rust consistently permits an extra tailing comma everywhere commas are used:
 * function arguments, arrays, struct and enum definitions, and so on.
 * There are tuples that contain a single value: the literal ("lonely hearts",) is a
 * tuple containing a single string; its type is (&str,). 
 * Here, the comma after the value is necessary to distinguish the singleton tuple
 * from a simple parenthetic expression.
 *
 *
 * Pointer Types
 * -------------
 *
