use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    println!(
        "{}",
        s[0].to_digit(10).expect("Invalid Input") * s[2].to_digit(10).expect("Invalid Input")
    );
}
