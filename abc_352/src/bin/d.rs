use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        p: [usize; n],
    }

    let mut idx = vec![0; n];

    for (i, pi) in p.iter().enumerate() {
        idx[pi - 1] = i;
    }

    let mut set = BTreeSet::new();

    for i in idx.iter().take(k) {
        set.insert(*i);
    }

    let mut ans = diff(&set);

    for i in 1..=(n - k) {
        set.remove(&idx[i - 1]);
        set.insert(idx[i + k - 1]);
        ans = ans.min(diff(&set));
    }

    println!("{ans}");
}

fn diff(set: &BTreeSet<usize>) -> usize {
    set.last().expect("cannot be empty") - set.first().expect("cannot be empty")
}
