use std::{cmp::Ordering, collections::HashMap};

use proconio::input;

struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return;
        }
        match self.rank[root_x].cmp(&self.rank[root_y]) {
            Ordering::Less => {
                self.parent[root_x] = root_y;
            }
            Ordering::Greater => {
                self.parent[root_y] = root_x;
            }
            _ => {
                self.parent[root_y] = root_x;
                self.rank[root_x] += 1;
            }
        }
    }
}

fn main() {
    input! {
       n: usize, m: usize,
       edges: [(usize, usize); m],
    }

    let mut uf = UnionFind::new(n);
    for (a, b) in edges {
        uf.union(a - 1, b - 1);
    }

    let group_count = (0..n)
        .map(|i| uf.find(i))
        .fold(HashMap::new(), |mut acc, x| {
            *acc.entry(x).or_insert(0) += 1;
            acc
        });
    let available_edge_count = group_count.values().map(|&x| x - 1).sum::<usize>();

    println!("{}", m - available_edge_count);
}
