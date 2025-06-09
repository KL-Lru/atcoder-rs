use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize, d: usize,
        s: Chars,
    }

    let mut t = Vec::new();
    let mut cnt = 0;

    for ci in s.iter().rev() {
        if *ci == '@' && cnt < d {
            t.push('.');
            cnt += 1;
        } else {
            t.push(*ci);
        }
    }

    println!("{}", t.iter().rev().collect::<String>());
}
