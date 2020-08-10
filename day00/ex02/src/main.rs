use std::env::args;
use std::str::FromStr;

fn main() {
    let start: Option<isize> = args().nth(1).and_then(|s| FromStr::from_str(&s).ok());
    let stop: Option<isize> = args().nth(2).and_then(|s| FromStr::from_str(&s).ok());

    match (start, stop) {
        (Some(start), Some(stop)) => {
            for x in start..stop {
                println!("{}", x);
            }
        },
        _ => println!("Invalid or not enough bounds provided!"),
    }
}
