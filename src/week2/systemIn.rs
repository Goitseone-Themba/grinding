use std::io;

fn main() {

    //prompt the user to enter their name
    println!("What is you name?");

    //create a mutable string to store the user's name
    let mut input = String::new();

    //use the io namespace's stdin to read string input from the user
    io::stdin().read_line(&mut input).expect("error: unable to read user input");

    //print the user's string input
    println!("hello, {}", input);
}