use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        skills: [(usize, usize); n],
    }

    let mut graph = HashMap::<usize, Vec<usize>>::new();
    let mut stack = vec![];

    for (idx, &(ai, bi)) in skills.iter().enumerate() {
        if ai == 0 && bi == 0 {
            stack.push(idx);
            continue;
        }

        for k in [ai - 1, bi - 1] {
            graph
                .entry(k)
                .and_modify(|edge| edge.push(idx))
                .or_insert(vec![idx]);
        }
    }

    let mut ans = HashSet::<usize>::new();

    while let Some(v) = stack.pop() {
        ans.insert(v);

        for dist in graph.get(&v).unwrap_or(&vec![]) {
            if ans.contains(dist) {
                continue;
            }

            stack.push(*dist);
        }
    }

    println!("{}", ans.len());
}
