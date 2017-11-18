// Pointers in Rust

fn main() {
	let health = 32;
	let mut game = "Space Invaders";

	println!("address of   game-value: {:p}", &game);
	println!("address of health-value: {:p}", &health);


	let game2 = &game;
	println!("address of   game2-value: {:p}", game2);

	println!("The contents of address {:p} is {}", game2, *game2);
	println!("The contents of address {:p} is {}", &game, game);
}

/* 


	The output is:

	address of   game-value: 0x4269fff7d8
	address of health-value: 0x4269fff7d4
	address of   game2-value: 0x4269fff7d8
	The contents of address 0x4269fff7d8 is Space Invaders
	The contents of address 0x4269fff7d8 is Space Invaders


*/
