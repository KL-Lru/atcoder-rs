use std::cmp::Reverse;
use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
      n: usize, m: usize,
      a: [usize; n],
      edge: [(usize, usize, usize); m],
    }

    let mut edge_map = HashMap::new();
    for (u, v, w) in edge {
        let u = u - 1;
        let v = v - 1;
        edge_map.entry(u).or_insert(vec![]).push((v, w + a[v]));
        edge_map.entry(v).or_insert(vec![]).push((u, w + a[u]));
    }

    let mut ans = vec![usize::MAX; n];
    let mut used = vec![false; n];
    let mut pq = std::collections::BinaryHeap::new();

    for (v, w) in &edge_map[&0] {
        pq.push(Reverse((*w, *v)));
    }

    while !pq.is_empty() {
        let Reverse((w, v)) = pq.pop().expect("pq should not be empty");
        if used[v] {
            continue;
        }
        used[v] = true;

        ans[v] = w;
        for (u, dw) in &edge_map[&v] {
            if !used[*u] {
                pq.push(Reverse((w + dw, *u)));
            }
        }
    }

    println!(
        "{}",
        ans.iter()
            .skip(1)
            .map(|&x| (x + a[0]).to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
