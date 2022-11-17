fn main() {
	let p = 520_000_000;
	let n = 5;
	let r = 10;

	//to find compund interest
	let a = p * (1+(r/100))^n;
	let ci = a-p;
	println!("Compund interest is {}",ci);
}



