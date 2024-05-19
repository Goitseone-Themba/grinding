use std::io;

fn main() {
    
    let principle: f64;
    let rate: f64;
    let time: f64;

    let interest: f64;
    let total_amount: f64;

    let mut input = String::new();
    
    println!("****Welcome to simple interest calculator****");
    
    println!("Enter a principle amount:");
    io::stdin().read_line(&mut input).expect("error: cannot read the provided principle amount");
    principle = input.trim().parse().expect("error: provided principle not a number");

    input.clear();

    println!("Enter interest rate in %:");
    io::stdin().read_line(&mut input).expect("error: cannot read the provided interest rate");
    rate = input.trim().parse().expect("error: provided rate in not a number");

    input.clear();

    println!("Enter time in years:");
    io::stdin().read_line(&mut input).expect("error: cannot read the provided time");
    time = input.trim().parse().expect("error: provided time in not a number");
    
    interest = (principle * rate * time)/100.0;
    total_amount = principle + interest;

    println!("Interest earned: P{}", interest);
    println!("Total Amount: P{}", total_amount);
    
}