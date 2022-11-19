//Rust Program to calculate the area of a trianlge given three sides

use std::io;

fn main(){

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Ënter first side of triangle:");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter second side of triangle:");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");


    println!("Enter third side of triangle:");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let s:f32 = (a*b*c)/2.0;
    let mut area:f32 = s* (s-a) * (s-b) * (s-c);
    area = area.sqrt();

    println!("Area of triangle: {}", area);
}
