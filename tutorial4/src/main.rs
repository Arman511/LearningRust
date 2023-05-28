use std::io;

fn main() {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let int_input: i64 = input.trim().parse().unwrap(); //This is how to parse a string to number


    println!("{}", int_input+2);

    io::stdin().read_line(&mut input).expect("Failed to read line");


}
