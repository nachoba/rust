

fn main() {
	let languages: [&str; 5] =
			["English", "Spanish", "Chinese", "French", "German"];

	println!("I know the following languages {0}, {1}, {2}, {3}, and {4}",
			languages[0], languages[1], languages[2], languages[3], languages[4]);

	println!("So you know {} languages?", languages.len());

	println!("Yes I know:");

	for i in 0..languages.len() {
		println!("\t\t{}", languages[i]);
	}

	println!("So your favorite language is? ");
	
	let mut favorite = String::new();

	println!("{:?}", std::io::stdin().read_line(&mut favorite));
	println!("Ok, its {}", &favorite);

}
