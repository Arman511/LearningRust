fn main() {
    let cond = (2 as f32)>3.0;
    println!("{}", cond);


    let cond2 = true && cond;
    println!("{}", cond2);
    let cond2 = true || cond;
    println!("{}", cond2);
    let cond2 =  ! cond;
    println!("{}", cond2);
    println!("Hello, world!");
    // ! happen first then && then ||
    size
}
