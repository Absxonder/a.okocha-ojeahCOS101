// Rust program to determine the annual incentive of an employee

use std::io;

fn main() {

let mut input1 = String::new();
let mut input2 = String::new();

println!("Are you experienced?, True/false: ");
io::stdin().read_line(&mut input1).expect("Not a valid string");
let experience:bool = input1.trim().parse().expect("You can only type true or false");

println!("Enter your age");
io::stdin().read_line(&mut input2).expect("Not a valid string");
let age:i32 = input2.trim().parse().expect("Not a valid number");

if experience == true && age >= 40 {

 
    println!("Your incentive is 1_560_000 per month");

}

 else if experience == true && age >= 30 && age < 40 {
 

    println!("Your incentive #1_480_000 per month");
}

 else if experience == true && age < 28 {


    println!(" Your incentive is #1_300_000 per month");
}

else if experience == false {

    println!("Your incentive is 100_000 per month");
}

}