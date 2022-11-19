//Project to find the roots of a quadratic equation

use std::io;

fn main(){

	let mut input1 = String::new();
	let mut input2 = String::new();
	let mut input3 = String::new();

	println!("\nEnter the first number:");
	io::stdin().read_line(&mut input1).expect("Failed to read input, try again");
	let a:i32 = input1.trim().parse().expect("Failed to read input");
   
   println!("\nEnter the second number:");
   io::stdin().read_line(&mut input2).expect("Failed to read input, try again");
   let b:i32 = input2.trim().parse().expect("Failed to read input, try again");

   println!("\nEnter the third number:");
   io::stdin().read_line(&mut input3).expect("Failed to read input, try again");
   let c:i32 = input3.trim().parse().expect("Failed to read input, try again");

   let dis:i32 = b^2 - (4*a*c);

   if dis>0{
   	println!("There are two distinct roots");
   }
   if dis == 0{
   	println!("There is exactly 1 root");
   }
   if dis <0{
   	println!("There are no real roots");
   }


}