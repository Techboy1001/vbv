use std::io;


fn main() {
    println!(" it is program to calculate distance between 
     two points......please Enter cordinates of point1(x1,y1) and point2(x2,y2)");

println!("Enter x1");
let mut x1 = String::new();
io::stdin().read_line(&mut x1)
.expect("Error getting ");
let x1:f32=x1.trim().parse().unwrap();

println!("Enter y1");
let mut y1 = String::new();
io::stdin().read_line(&mut y1)
.expect("Error getting ");
let y1:f32=y1.trim().parse().unwrap();

println!("Enter x2");
let mut x2 = String::new();
io::stdin().read_line(&mut x2)
.expect("Error getting ");
let x2:f32=x2.trim().parse().unwrap();

println!("Enter y2");
let mut y2 = String::new();
io::stdin().read_line(&mut y2)
.expect("Error getting ");
let y2:f32=y2.trim().parse().unwrap();

let distance =((x2-x1).powi(2) + (y2-y1).powi(2)) .powf(0.5);

println!("distance is {}",distance);
}