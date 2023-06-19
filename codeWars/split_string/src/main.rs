fn camel_case(text: &str) -> String {
    text.split_whitespace().map(|word| {
        let mut chars = word.chars();
        match chars.next() {
            Some(first) => {
                let rest: String = chars.collect();
                let cap: String = first.to_uppercase().to_string();
                cap + &rest
            }
            None => String::new(),
        }
    }).collect()
}

fn main() {
    println!("{:?} expected {}", camel_case("test case"), "TestCase");
}
