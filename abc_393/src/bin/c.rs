use std::collections::HashSet;

use proconio::input;

fn swap(edge: (usize, usize)) -> (usize, usize) {
    if edge.0 < edge.1 {
        edge
    } else {
        (edge.1, edge.0)
    }
}

fn main() {
    input! {
      _: usize, m: usize,
      edges: [(usize, usize); m]
    }

    let mut set = HashSet::new();
    let mut ans = 0;

    for e in edges {
        if e.0 == e.1 {
            ans += 1;
            continue;
        }

        let e = swap(e);
        if set.contains(&e) {
            ans += 1;
        } else {
            set.insert(e);
        }
    }

    println!("{ans}");
}
