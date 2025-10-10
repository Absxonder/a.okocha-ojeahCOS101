fn main() {
	let p:f64 = 510000.0;
	let r:f64 = 5.0;
    let n:i32 = 3;

    let a:f64 = p * (1.0-(r/100.0)).powi(n);

    println!("The value of the TV after three years is {}", a);

}