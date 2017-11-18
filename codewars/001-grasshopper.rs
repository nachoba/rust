/*	https://www.codewars.com/kata/586c1cf4b98de0399300001d/train/rust
	Grasshopper
	-----------
	Create a combat function that takes the player's current health and the
	amount of damage received, and returns the player's new health. Health
	can't be less than 0.

 */


fn combat(health: f32, damage: f32) -> f32 {
	let state: f32 = health - damage;
	if state < 0.0 {
		return 0.0;
	} else {
		return state;
	}
}


fn main() {
	let salud: f32 = 20.0;
	let danio: f32 = 30.0;

	let state: f32 = combat(salud, danio);
	println!("With a health of {0} and a damage of {1} your status is {2}",
				salud, danio, state);

}


/*	A clever solution given is:

	fn combat(health: f32, damage: f32) -> f32 {
		(health - damage).max(0.0)
	}

 */

