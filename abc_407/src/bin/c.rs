use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    let mut buf = 0;
    let mut ans = s.len() as u32;
    while let Some(c) = s.pop() {
        let n = c.to_digit(10).expect("must be number");

        ans += (10 + n - buf) % 10;
        buf = n;
    }

    println!("{ans}");
}
