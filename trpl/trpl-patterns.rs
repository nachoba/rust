/*

	PATTERNS
	--------
	Patterns are quite common in Rust. We use them in variable bindings, match
	expressions, and other places too.
 */


fn main() {

	// You can match against literals directly and _ acts as an 'any' case:
	let x = 1;

	match x {
		1	=>	println!("x: one"),
		2	=>	println!("x: two"),
		3	=>	println!("x; three"),
		_	=>	println!("x; anything"),
	}

	// It's possible to create a binding for the value in the any case:
	// But note it is an error to have both a catch-all _ and a catch all
	// binding in the same match block:
	//		y 	=> .....
	//		_	=> .....
	match x {
		y 	=>	println!("x: {} y: {}", x, y),
	}

	// There's one pitfall with patterns: like anything that introduces a new
	// binding, they introduces shadowing. For example:
	let x1	=	1;
	let x2	=	'c';

	match x2 {
		x1 	=>	println!("x1: {} x2: {}", x1, x2),
	}
	println!("x1: {}", x1);

	// In other words, x1 => matches the pattern and introduces a new binding
	// named x1. This new binding is in scope for the match arm and takes on
	// the value of x2.
	// Notice that the value of x1 outside the scope of the match has no
	// bearing on the value of x1 within it.


	// Multiple patterns: You can match multiple patterns with |
	let z = 1;
	match z {
		1 | 2	=> println!("z is one or two"),
		    3	=> println!("z is three"),
		    _	=> println!("z is anything"),
	}

}

