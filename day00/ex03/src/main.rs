use std::env::args;
use std::str::FromStr;

fn main() {
    let start: Option<isize> = args().nth(1).and_then(|s| FromStr::from_str(&s).ok());
    let stop: Option<isize> = args().nth(2).and_then(|s| FromStr::from_str(&s).ok());

    match (start, stop) {
        (Some(start), Some(stop)) => {
            let sum: isize = (start..stop).sum();
            println!("The sum of all the numbers between {} and {} is {}.", start, stop, sum);
        },
        _ => println!("Invalid or not enough bounds provided!"),
    }
}
