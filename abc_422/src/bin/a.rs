use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let world = s[0].to_digit(10).expect("Must be digit");
    let stage = s[2].to_digit(10).expect("Must be digit");

    if stage == 8 {
        println!("{}-1", world + 1);
    } else {
        println!("{world}-{}", stage + 1);
    }
}
