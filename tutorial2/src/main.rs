use std::io ;


fn main() {
    println!("Hello, world!");
    let mut input = String::new();//has to be a string when inputting 
    io::stdin().read_line(&mut input).expect("Failed to read line");

    println!("{}",input);
}
