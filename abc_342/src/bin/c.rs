use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        mut s: Chars,
        q: usize,
        op: [(char, char); q]
    }

    let mut mapping = HashMap::new();

    for c in 'a'..='z' {
        mapping.insert(c, c);
    }

    for (src, dst) in op {
        for c in 'a'..='z' {
            if mapping[&c] == src {
                mapping.insert(c, dst);
            }
        }
    }

    let mut ans = vec!['?'; s.len()];

    for (src, dst) in mapping {
        for (i, c) in s.iter().enumerate() {
            if c == &src {
                ans[i] = dst;
            }
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
