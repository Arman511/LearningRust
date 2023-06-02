fn main() {
    let  arr_it =[1,2,3,4];
    for i in arr_it.iter(){
        println!("{}",i);
    }
    let number = 42;
    let text = "Hello, Rust!";
    let tuple = (1, "two", 3.0);

    println!("Number: {:?}", number);
    println!("Text: {:?}", text);
    println!("Tuple: {:?}", tuple);
    println!("Tuple: {:?}", arr_it);

    println!("Number: {}", number);
    println!("Text: {}", text);
    
}
