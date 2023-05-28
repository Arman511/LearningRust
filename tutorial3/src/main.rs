fn main() {
    let x = 255f32;
    println!("{}", x);
    let x = 255_f32;
    println!("{}", x);
    let x = 255_000 as f32;
    let y = 10f32;

    let z = x / y;
    println!("{}", z);
    let mut a: u8;
    a = 8;
    println!("{}", a);
    a = 10;
    println!("{}", a);

    let x = 127000 as i64;
    let y = 10i32;

    let z = x / (y as i64);
    println!("{}", z);

    let x = i32::MAX;
    println!("{}", x);
    let x = i64::MAX;
    println!("{}", x);
    let x = i128::MAX;
    println!("{}", x);

    let log_10 = i128::MAX.ilog10();
    println!("{}", log_10);
}
