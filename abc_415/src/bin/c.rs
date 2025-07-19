use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! { t: usize  }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! { n: usize, s: Chars }

    let mut layers = vec![HashSet::new(); n + 1];
    let ln = (0..n).map(|i| 1 << i).collect::<Vec<_>>();

    layers[0].insert(0);

    for i in 1..=n {
        let prev_layer = &layers[i - 1];
        let mut current_layer = HashSet::new();

        for &x in prev_layer {
            for y in 0..n {
                if x & ln[y] != 0 {
                    continue;
                }
                if s[(x | ln[y]) - 1] == '0' {
                    current_layer.insert(x | ln[y]);
                }
            }
        }
        layers[i] = current_layer;
    }

    if layers[n].is_empty() {
        println!("No");
    } else {
        println!("Yes");
    }
}
