fn main() {
    println!("Hello, world!");
    let result: i32 = add_number(2, 3);
    println!("{}", result)
}

fn add_number(x: i32, y:i32) -> i32{
    let result: i32 = x+y;
    result
}


