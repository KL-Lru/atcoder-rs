use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ans = vec![];
    let mut si = 0;
    let mut ti = 0;

    while si < s.len() {
        while s[si] != t[ti] {
            ti += 1;
        }

        ans.push((ti + 1).to_string());
        si += 1;
        ti += 1;
    }

    println!("{}", ans.join(" "));
}
