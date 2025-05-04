use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        vec: [(usize, usize); m],
    }

    if n != m {
        println!("No");
        return;
    }

    let edge_map = make_edge_map(&vec);

    if !is_two_dist(&edge_map) {
        println!("No");
        return;
    }

    if is_connected(n, &edge_map) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn make_edge_map(vec: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut edge_map = HashMap::<usize, Vec<usize>>::new();
    for (a, b) in vec {
        edge_map.entry(*a).or_default().push(*b);
        edge_map.entry(*b).or_default().push(*a);
    }
    edge_map
}

fn is_two_dist(edge_map: &HashMap<usize, Vec<usize>>) -> bool {
    edge_map.values().all(|edges| edges.len() == 2)
}

fn is_connected(n: usize, edge_map: &HashMap<usize, Vec<usize>>) -> bool {
    let mut visited = HashSet::new();
    let first_node = 1;

    dfs(first_node, edge_map, &mut visited);

    visited.len() == n
}

fn dfs(node: usize, edge_map: &HashMap<usize, Vec<usize>>, visited: &mut HashSet<usize>) {
    if visited.contains(&node) {
        return;
    }
    visited.insert(node);

    for &next_node in &edge_map[&node] {
        if !visited.contains(&next_node) {
            dfs(next_node, edge_map, visited);
        }
    }
}
