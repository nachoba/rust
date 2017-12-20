/*
 * THE RUST PROGRAMMING LANGUAGE
 * CHAPTER 3    :: VECTORS 
 * -----------------------------------------------------------------------------
 * A 'vector' is a dynamic or growable array, implemented as the standard libra-
 * ry type Vec<T>. The T means that we can have vectors of any type -meaning that
 * they are generic.
 *
 * Vectors always allocate their data on the heap. You can create them with the
 * "vec!" macro.
 */


fn create_vector() {
    let v = vec![1, 2, 3, 4, 5];

    /* There's an alternate form of "vec!" for repeating an initial value.
     * This creates a vector of ten zeros:
     */
    let b = vec![0; 10];
    /* Vectors store their contents as contiguous arrays of T on the heap. This
     * means that they must be able to know the size of T at compile time (how
     * many bytes are needed to store a T). The size of some things can't be
     * known at compile time. For these you'll have to store a pointer to that
     * thing: thankfully, the "Box" type works perfectly for this.
     */


/* Accessing Elements
 * ------------------
 * To get the value at a particular index in the vector, we use "[]"
 */
    
    println!("The third element of v is {}", v[2]);
    println!("The last  element of b is {}", b[b.len() - 1]);

/* The indices count from 0, so the third element of v is v[2]. It's also im-
 * portant to note that you must index with the "usize" type. Indexing with a
 * non-usize type gives an error.
 */

    for i in 0..v.len() {
        println!("The element at {} is {}", i, v[i]);
    }

}




fn main() {
    create_vector();
}

