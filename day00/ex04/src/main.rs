use std::env::args;
use std::str::FromStr;

fn average(numbers: &[i32]) -> Option<f64> {
    match numbers.len() {
        0   => None,
        len => {
            let sum: isize = numbers.iter().map(|&n| n as isize).sum();
            Some(sum as f64 / len as f64)
        },
    }
}

fn main() {
    match args().nth(1) {
        Some(numbers) => {
            let numbers: Result<Vec<i32>, _> = numbers.split(',').map(FromStr::from_str).collect();

            match numbers {
                Ok(numbers) => {
                    match average(&numbers) {
                        Some(avg) => println!("The average is {}.", avg),
                        None => println!("Not enough numbers."),
                    }
                },
                Err(e) => eprintln!("{:?}", e),
            }
        },
        None => println!("Give me more"),
    }
}

#[test]
fn test_average_easy() {
    let avg = average(&[1, 2, 3]);
    assert!(avg == Some(2.0));
}

#[test]
fn test_average_empty() {
    let avg = average(&[]);
    assert!(avg == None);
}
