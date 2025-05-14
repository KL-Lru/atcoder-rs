use core::panic;
use proconio::input;

fn main() {
    input! {
        r: usize, x: u8,
    }

    match (x, r) {
        (1, 1600..=2999) => println!("Yes"),
        (1, _) => println!("No"),
        (2, 1200..=2399) => println!("Yes"),
        (2, _) => println!("No"),
        _ => panic!("Invalid input"),
    }
}
