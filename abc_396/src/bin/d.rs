use std::ops::BitXor;

use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! {
        n:usize, m :usize,
        edges: [(usize,usize,usize);m],
    }
    let mut graph = vec![vec![]; n];
    for (a, b, c) in edges {
        graph[a - 1].push((b - 1, c));
        graph[b - 1].push((a - 1, c));
    }

    println!("{}", calc(&graph, 0));
}

fn calc(graph: &Vec<Vec<(usize, usize)>>, start: usize) -> usize {
    let mut visited = vec![false; graph.len()];
    visited[start] = true;
    let mut ans = INF;
    for &(next, cost) in &graph[start] {
        ans = ans.min(dfs(graph, next, &mut visited, cost));
    }
    ans
}

fn dfs(
    graph: &Vec<Vec<(usize, usize)>>,
    node: usize,
    visited: &mut Vec<bool>,
    xor: usize,
) -> usize {
    if node == graph.len() - 1 {
        return xor;
    }

    visited[node] = true;

    let mut ans = INF;

    for &(next, cost) in &graph[node] {
        if !visited[next] {
            ans = ans.min(dfs(graph, next, visited, xor.bitxor(cost)));
        }
    }

    visited[node] = false;

    ans
}
