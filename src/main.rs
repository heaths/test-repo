use std::env;

fn main() {
    let name = env::var("USER")
        .or_else(|_| env::var("USERNAME"))
        .unwrap_or_else(|_| String::from("world"));
    println!("Hello, {name}!");
}
