use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: usize,
        query: [(usize, usize); q],
    }

    let mut map = HashMap::new();

    for (i, &pi) in p.iter().enumerate() {
        map.insert(pi, i);
    }

    for (ai, bi) in query {
        if map[&ai] < map[&bi] {
            println!("{ai}");
        } else {
            println!("{bi}");
        }
    }
}
