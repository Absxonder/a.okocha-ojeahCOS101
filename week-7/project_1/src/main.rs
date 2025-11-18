use std::io;

// Function to read a number from the user
fn read_input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// Area of trapezium
fn area_trapezium() {
    let height = read_input("Enter the height:");
    let base1 = read_input("Enter base1:");
    let base2 = read_input("Enter base2:");
    let area = height / 2.0 * (base1 + base2);
    println!("Area of the trapezium is: {}", area);
}

// Area of rhombus
fn area_rhombus() {
    let d1 = read_input("Enter diagonal 1:");
    let d2 = read_input("Enter diagonal 2:");
    let area = 0.5 * d1 * d2;
    println!("Area of the rhombus is: {}", area);
}

// Area of parallelogram
fn area_parallelogram() {
    let base = read_input("Enter the base:");
    let altitude = read_input("Enter the altitude:");
    let area = base * altitude;
    println!("Area of the parallelogram is: {}", area);
}

// Area of cube
fn area_cube() {
    let side = read_input("Enter the length of the side:");
    let area = 6.0 * side.powi(2);
    println!("Area of the cube is: {}", area);
}

// Volume of cylinder
fn volume_cylinder() {
    let radius = read_input("Enter the radius:");
    let height = read_input("Enter the height:");
    let volume = std::f64::consts::PI * radius.powi(2) * height;
    println!("Volume of the cylinder is: {}", volume);
}

fn main() {
    println!("Select a shape to calculate:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    let choice = read_input("Enter your choice (1–5):") as i32;

    match choice {
        1 => area_trapezium(),
        2 => area_rhombus(),
        3 => area_parallelogram(),
        4 => area_cube(),
        5 => volume_cylinder(),
        _ => println!("Invalid choice!"),
    }
}
