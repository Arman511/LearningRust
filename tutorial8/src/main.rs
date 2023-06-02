use std::fs::File;
use std::io::{BufRead, BufReader, ErrorKind, Write};

fn main() {
    let path = "Lines.txt";
    let output = File::create(path);
    let mut output = match output {
        Ok(file) => file,
        Err(f) => panic!("Problem creating file ; {:?}", f),
    };
    write!(output, "Just some\nwords").expect("Failed top write to file");

    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    let o2 = File::create("rand.txt");
    let o2 = match o2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(f) => panic!("Problem creating file ; {:?}", f),
            },
            _other_error => panic!("Could not create"),
        },
    };
    println!("{:?}",o2)
}
