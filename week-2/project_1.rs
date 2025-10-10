fn main() {
	let p:i64 = 520000000;
	let r:i64 = 10;
	let n:i64 = 5;
	let a:i64 = p * (1 + (r/100)) * n;
	println!("The amount is {}", a);

	let ci = a - p;
	println!("The compound interest is {}", ci);
}