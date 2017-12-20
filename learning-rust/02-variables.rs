/*
 


        LEARNING RUST
        =============
        Chapter 2   ::  Variables
        -------------------------
        In this chapter we will do the following:

        * Undesrtand variable mutability
        * See how Rust stores information in a variable, and the types of varia-
          ble available
        * See how Rust deals with vector variable types
        * Understand how Rust can and cannot manipulate variables
        * See how Rust can pass variables
        * Take a look at how Rust stores a variable internally




    Variable Mutability
    -------------------
    Rust defaults to non-mutability of variables.  That means that variable bin-
    dings are actually constants if not explicitly defined as mutable.

    Creating a Variable
    -------------------
    To create a new variable binding in Rust, we use the following form:

        let x = 1;

    By default, all variables in Rust are non-mutable; therefore, we have to ex-
    plicitly define a variable as being mutable.

    Const and Static
    ----------------
    Rust has two types of constants: "consts" and "statics".  Consts are sort of
    like aliases: their contents are sort of replaced on the place where they're
    used. The syntax is like this:

        const PI: f32 = 3.1415922;

    Statics are more like variables. They have a global scope of the program and
    are defined as follows:

        static MY_VARIABLE: i32 = 255;

    They cannot be altered.Rust is able to guess the types of local function va-
    riables. This is called local type inference. However, it is only local,  so
    types of statics and consts must always be typed out.

    Defining the Variable Value before Use
    --------------------------------------
    While it is not enforced in some languages,  a variable must have an initial
    value in Rust, even if it is zero.

    Strings
    -------
    Typically, a string can be defined in one of two ways:

        let my_name = "Nacho";

    This is known as a string slice. 

    The second way is to use String::new();

    This is a String, with capital S.  It is allocated in the heap and can  grow
    dynamically.


    How Rust uses Memory
    --------------------
    The memory occupied by any Rust program is split into two distinct areas the
    heap and the stack. Simply put, the stack contains primitive variables,while
    the heap stores complex types. A heap can grow and grow until the  available
    memory is exhausted.
    The stack is faster and simpler but may not grow without limits.  Every bin-
    ding in Rust is in a stack,  but those bindings may  refer to things  in the
    heap, and elsewhere.


    This all relates directly to the string example. The binding "my_name" is in
    the stack, and refers to a literal static string "Nacho".  That string being
    static means that is is somewhere in memory when the program starts and  its
    being static also means it cannot be changed.

    "String::new();", on the other hand, creates a String in the heap.  It is i-
    nitially empty, but may grow to fill the whole virtual memory space.  One of
    the ways of creating Strings is to call the ".to_owned()" method on a string
    slice:

        let mut my_string_one = "This is my first string ".to_owned();
        let my_string_two     = "This is my second string. ";
        let my_string_three   = "This is my final string.";

        my_string_one = my_string_one + my_string_two + my_string_three +
                        my_string_two;

    There are other ways, but this is the most recommended one because it under-
    lines the ownership issue.

    The binding "my_string_one" starts out at 24 character long,and would be al-
    located at least that size on the heap; that binding is actually a reference
    to the position on the heap where my_string_one lives.

