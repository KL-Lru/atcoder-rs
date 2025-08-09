use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut map = HashMap::new();

    for (idx, c) in s.iter().enumerate() {
        map.entry(c).or_insert(vec![]).push(idx + 1);
    }

    let ans = map
        .iter()
        .find(|(_, v)| v.len() == 1)
        .and_then(|(_, v)| v.first())
        .expect("Cannot empty");

    println!("{ans}");
}
