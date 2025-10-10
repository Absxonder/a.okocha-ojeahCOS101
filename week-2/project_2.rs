fn main() {
let t:i64 = 2 * 450000;
let m:i64 = 1 * 1500000;
let h:i64 = 3 * 750000;
let d:i64 = 3 * 2850000;
let a:i64 = 1 * 250000;

let s = t + m + h + d + a; 

let avg = s / 5;

println!("The sum of the sales record is {}", s);	

println!("The average of the sales record is {}", avg);
}