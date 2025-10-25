// Rust program to find the roots of a quadratic equation

use std::io;

fn main() {

    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter your first number: ");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter your second number ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid string number");

    println!("Enter your third number ");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    let discriminant = b.powf(2.0) - 4.0 * a * c;

    if discriminant >= 0.0 {

    let root1:f32 = (-b + discriminant.sqrt()) / (2.0 * a);
    let root2:f32 = (-b - discriminant.sqrt()) / (2.0 * a);



    println!("The roots of the equation are {} and {}", root1, root2);

    }

    else {
        print!("The equation has complex roots");
    }

}