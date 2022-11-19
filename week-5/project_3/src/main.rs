//Rust program tha dusplays menu, prices and discount

use std::io;

fn main(){
	let p:i32 = 3200;
	let f:i32 = 3000;
	let a:i32 = 2500;
	let e:i32 = 2000;
	let w:i32 = 2500;


println!("p - Poundo Yam/Edikaiko Soup - N{}", p);
println!("f - Fried Rice and Chicken - N{}", f);
println!("a - Amala and Ewedu Soup - N{}", a);
println!("e - Eba and Egusi Soup - N{}", e);
println!("w - White Rice and Stew - N{}",w);

let mut input1 = String::new();
let mut input2 = String::new();

println!("What would you like to order?");
io::stdin().read_line(&mut input1).expect("Failed to read input");
 
 

/*println!("How many portions?");
io::stdin().read_line(&mut input2).expect("Failed to read input");
let mut por:i32 = input2.trim().parse().expect("Failed to read input");

//To find total charges
let mut tot:i32 = por * choice;

//Incomplete/ Not sure of code*/
}



