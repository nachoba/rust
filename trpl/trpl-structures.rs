/*
 *  STRUCTURES
 *  ----------
 *  "struct" are a way of creating more complex data types. For example, if we
 *  were doing calculations involving coordinates in 2D space, we would need
 *  both an x and a y value:
 *
 *      let origin_x = 0;
 *      let origin_y = 0;
 *
 *  A "struct" lets us combine these two into a single, unified datatype with
 *  x and y as field labels.
 */

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let origin: Point = Point {x: 0, y: 0};

    println!("The origin is at ({},{})", origin.x, origin.y);


/*
    We declare a "struct" with the "struct" keyword, and then with a name. By
    convention structs begin with a capital letter and are camel cased:
        
            PointInSpace
            EmployeeRecord

   We can create an instance of our struct via "let", but we use a "key: value"
   style syntax to set each field. The order doesn't need to be the same as in
   the original declaration.
   Because fileds have names, we can access them through dot notation: origin.x

   The values in structs are immutable by default, like other bindings in Rust.
   We use "mut" to make them mutable:
 */

    let mut point: Point = Point { x: 3, y: 5};
    println!("The point is at ({},{})", point.x, point.y);
    point.x = 5;
    println!("The point moved to ({},{})", point.x, point.y);



/*
    Rust does not support field mutability at the language level, so you cannot
    write something like this:

        struct Point {
            mut x: i32,     // This causes an error
                y: i32,
        }

   Mutability is a property of the binding, not of the structure itself. This
   lets you make things mutable on a temporary basis:

 */

    let mut another_point: Point = Point { x: 3, y: 3 };
    another_point.x = 5;
    let another_point = another_point;  // `another_point` is now immutable
    // another_point.y = 6;   would cause an error.
    println!("another_point is ({}:{})", another_point.x, another_point.y);


} 
