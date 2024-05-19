use std::io;

fn main() {
    
    let length: i32;
    let width: i32;

    //this mutable string holds user input so I can parse it into what ever I need it to be, like an i32 or f64
    let mut input = String::new();

    println!("****Welcome to area and perimeter calculator****\n");
    
    println!("enter length and width");

    //take input and parse it into an integer
    io::stdin().read_line(&mut input).expect("error: cannot read the length provided");
    length = input.trim().parse().expect("error: length provided is not a integer");
    
    //clear the mut string I use for input, because stdin will append the variable not reinitialize it.
    input.clear();
    io::stdin().read_line(&mut input).expect("error: cannot read the width provided");
    width = input.trim().parse().expect("error: width provided is not an integer");

    let area: i32 = width * length;
    let perimeter: i32 = (length*2) + (width*2);

    println!("Area = {}", area);
    println!("Perimeter = {}", perimeter);
    
}