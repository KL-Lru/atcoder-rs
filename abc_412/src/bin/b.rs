use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let ts = HashSet::<char>::from_iter(t);
    let mut ss = HashSet::<char>::new();

    for i in 1..s.len() {
        if s[i].is_uppercase() {
            ss.insert(s[i - 1]);
        }
    }

    if ss.is_subset(&ts) {
        println!("Yes");
    } else {
        println!("No");
    }
}
