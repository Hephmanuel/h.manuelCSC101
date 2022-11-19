//PROGRAM TO FIND THE EXPERIENCE AND AGE OF EMPLOYEE TO FIND THE ANNUAL INCENTIVE

use std::io;

fn main(){

	let mut exp = String::new();
	let mut input2 = String::new();
    
    println!("How many years of experience do you have in thsi field?");
    io::stdin().read_line(&mut exp).expect("Invalid input");
    let exp:i32 = exp.trim().parse().expect("Invalid input");
    
    println!("Enter your age: ");
    io::stdin().read_line(&mut input2).expect("Invalid input");
    let age:i32 = input2.trim().parse().expect("Invalid input");

    if (exp >=0) && (age >= 40){
    	println!("Your incentive is N,560,000 per month")
    }
    else if (exp >= 0) && (age >=30) && (age < 40){
    	println!("Your incentive is N1,480,000 per month");
    }
    else if (exp >= 0) && (age < 28){
    	println!("Your incentive is N1,300,000 per month");
    }
    else if exp < 0{
    	println!("Your incentive is N100,000")
    } 
    else{
    	println!("Invalid input");
}
} 

