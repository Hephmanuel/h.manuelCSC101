fn main(){
	let p = 210_000;
	let n = 3;
	let r = 5;

//to find depreciation
let a = p * (1 - (r/100)) ^ n;
println!("Amount is {}",a);
}