use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut ans = 0;

    for i in 0..s.len() {
        let mut j = 1;
        while 2 * j + i < s.len() {
            if s[i] == 'A' && s[i + j] == 'B' && s[i + 2 * j] == 'C' {
                ans += 1;
            }
            j += 1;
        }
    }

    println!("{ans}");
}
