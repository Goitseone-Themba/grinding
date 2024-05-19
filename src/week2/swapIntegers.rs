use std::io;

fn main() {
    let x: i32;
    let y: i32;

    let mut input = String::new();

    println!("**** Let's swap two integers ****");

    println!("Enter x:");
    io::stdin().read_line(&mut input).expect("error: could not read the value provided");
    x = input.trim().parse().expect("error: the value provided is not an integer");

    input.clear();

    println!("Enter y:");
    io::stdin().read_line(&mut input).expect("error: could not read the provided value");
    y = input.trim().parse().expect("error: the value provided is not an integer");

    println!("Before swapping:");
    println!("x = {}", x);
    println!("y = {}", y);

    let x = x + y;
    let y = x - y;
    let x = x - y;

    println!("After swapping:");
    println!("x = {}", x);
    println!("y = {}", y);
}
