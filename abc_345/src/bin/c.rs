use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut map = HashMap::<char, usize>::new();

    for c in s {
        *map.entry(c).or_default() += 1;
    }

    let mut ans = 0;
    for c in 'a'..='z' {
        ans += map.get(&c).unwrap_or(&0)
            * (((c as u8 + 1) as char)..='z')
                .map(|x| map.get(&x).unwrap_or(&0))
                .sum::<usize>();
    }

    if map.values().any(|&x| x > 1) {
        ans += 1;
    }

    println!("{ans}");
}
