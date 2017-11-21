/*
	Destructuring
	-------------
	If you have a compound data type, like a "struct", you can destructure
	it inside a pattern:
 */


struct Point {
	x: i32,
	y: i32,
}



fn main() {
	let origin = Point {x: 0, y: 0};
	println!("The origin is {}:{}", origin.x, origin.y);

	match origin {
		Point {x, y}	=> println!("{},{}", x, y),
	}


	// We can use : to give a value a different name:
	match origin {
		Point {x: x1, y: y1} => println!("{},{}", x1, y1),
	}

	// If we only care about some of the values, we don't have to gie them
	// all names:
	let point = Point { x: 2, y: 3 };

	match point {
	    Point {x, ..} => println!("x is {}", x),
	}

	// You can do this kind of match on any member, not only the first:
	match point {
	    Point{y, ..} => println!("y is {}", y),
	}

	// This "destructuring" behavior works on any compound data type, like
	// tuples or enums.
}


