use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        a: usize, b: usize, c: usize, d: usize, e: usize, f: usize,
        g: usize, h: usize, i: usize, j: usize, k: usize, l: usize,
    }

    if is_over_wrap(range_set(a, d), range_set(g, j))
        && is_over_wrap(range_set(b, e), range_set(h, k))
        && is_over_wrap(range_set(c, f), range_set(i, l))
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn range_set(start: usize, end: usize) -> HashSet<usize> {
    HashSet::from_iter(start..=end)
}

fn is_over_wrap(s1: HashSet<usize>, s2: HashSet<usize>) -> bool {
    s1.intersection(&s2).collect::<Vec<_>>().len() > 1
}
