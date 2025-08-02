use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut set = HashSet::new();

    for i in 0..s.len() {
        for j in (i + 1)..=s.len() {
            set.insert(s[i..j].to_string());
        }
    }

    println!("{}", set.len());
}
