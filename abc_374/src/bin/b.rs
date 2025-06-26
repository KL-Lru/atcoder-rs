use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    for i in 0..(s.len().min(t.len())) {
        if s[i] != t[i] {
            println!("{}", i + 1);
            return;
        }
    }

    if s.len() != t.len() {
        println!("{}", (s.len() + 1).min(t.len() + 1));
        return;
    }

    println!("0");
}
