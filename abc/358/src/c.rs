use std::{cmp::min, collections::HashSet};

use proconio::input;

fn satisfy(combination: u32, s: &[String], m: usize) -> bool {
    let mut popcorns = HashSet::new();
    for (ni, pops) in s.iter().enumerate() {
        if combination & (1 << ni) > 0 {
            for (idx, pop) in pops.chars().enumerate() {
                if pop == 'o' {
                    popcorns.insert(idx);
                }
            }
        }
    }

    popcorns.len() == m
}

fn main() {
    input! {
      n: usize,
      m: usize,
      s: [String; n],
    }

    let mut ans = 100;

    for combination in 0..(1 << n) {
        if satisfy(combination, &s, m) {
            ans = min(ans, combination.count_ones());
        }
    }

    println!("{}", ans);
}
