use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize, l: usize, r: usize,
        s: Chars
    }

    if s[(l - 1)..r].iter().all(|c| *c == 'o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
