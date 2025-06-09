use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize, d: usize,
        s: Chars,
    }

    println!("{}", s.iter().filter(|&&c| c == '.').count() + d);
}
