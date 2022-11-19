use std::io;

fn checker(){
	let mut input = String::new();
	println!("Ënter a character:" );
	io::stdin().read_line(&mut input).expect(Ïnvalid input);
	let ch:char = input.trim().parse().expect("Invalid input");

	if ch >= '0' && ch <= '9'{
		println!("Character '{}' is a digit", ch);
	}
	else{
		println!("Character '{}' is not a digit");
		}
}

fn main(){
	//calling fnction
	println!("Welcome! This program checks whether a character contains 
		a digit or not");
	checker()
}