use std::env::args;

fn main() {
    match args().nth(1) {
        Some(name) => println!("Hello, {}!", name),
        None => println!("Hello, world!"),
    }
}
