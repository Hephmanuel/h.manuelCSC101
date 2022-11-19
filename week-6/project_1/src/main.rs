use std::io;

//Area of trapezium
fn a_trapezium() { 

	println!("This is a program to find the area of a trapezium");
	
	let  mut input1 = String::new();
	let mut input2 = String::new();
	let  mut input3 = String::new();


	println!("\nEnter the height:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let h:i32 = input1.trim().parse().expect("Invalid input");

	println!("Enter base 1:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let b1:i32 = input2.trim().parse().expect("Invalid input");

	println!("Enter base 2: ");
	io::stdin().read_line(&mut input3).expect("Failed to read input");
	let b2:i32 = input3.trim().parse().expect("Invalid input");

    let a = h/2*(b1+b2);

    println!("The area of the trapezium is {}", a);
}
fn a_rhombus(){
	println!("This is a program to find the are of a rhombus");
    
    let  mut input1 = String::new();
	let mut input2 = String::new();

	println!("Enter the length of diagonal 1:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let d1:f32 = input1.trim().parse().expect("Invalid input");

    println!("Enter the length of diagonal 2:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let d2:f32 = input2.trim().parse().expect("Invalid input");

    let a = 0.5*d1*d2;
    println!("The area of the rhombus is: {}",a);
}
fn a_parallelogram(){

	println!("This is a program to find the area of a parallelogram");
	
    let mut input1 = String::new();
	let  mut input2 = String::new();

	println!("Enter the length of base:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let b:i32 = input1.trim().parse().expect("Invalid input");

    println!("Enter the altitude:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let h:i32 = input2.trim().parse().expect("Invalid input");

    let a = b*h;
    println!("The area of the parallelogram is: {}",a);
}
fn a_cube(){

	println!("This is a program to find the volume of a cube");

    let mut input1 = String::new();
	

	println!("Enter the length of cube:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let l:i32 = input1.trim().parse().expect("Invalid input");

    let a = 6*(l^2);
    println!("The volume of the cube is: {}",a);
}
fn v_cylinder(){

	println!("This is a program to find the volume of a cylinder");
	
    let mut input1 = String::new();
	let mut input2 = String::new();

	println!("Enter the radius:");
	io::stdin().read_line(&mut input1).expect("Failed to read input");
	let r:i32 = input1.trim().parse().expect("Invalid input");

    println!("Enter the height:");
	io::stdin().read_line(&mut input2).expect("Failed to read input");
	let h:i32= input2.trim().parse().expect("Invalid input");
    let a = (22/7)*(r^2)*h;
    println!("The volume of the cylinder is: {}",a);
}

fn main(){
 println!("1 - Find area of trapezium \n2 -  Find area of rhombus \n3 - Find area of parallelogram \n4 - Find volume of a cube\n5 - Find volume of a cylinder");
let mut input = String::new();
println!("Select a number: {}", input);
   
   io::stdin().read_line(&mut input).expect("Failed to read input");
   let input:i32 = input.trim().parse().expect("Failed to read input"); 

	

if input == 1{
	a_trapezium();
}
else if input ==2{
	a_rhombus();
}
else if input == 3{
	a_parallelogram();
}
else if input ==4{
	a_cube();
}
else if input ==5{
	v_cylinder();

}
else {
	println!("Invalid input")
}

 


}
