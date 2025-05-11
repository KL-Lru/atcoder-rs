use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
     n: usize,
      p: [usize; n],
    }

    let ranks = rank_map(&p);

    for pi in p {
        println!("{} ", ranks[&pi]);
    }
}

fn count_presences(p: &[usize]) -> HashMap<usize, usize> {
    p.iter().fold(HashMap::new(), |mut acc, &x| {
        *(acc.entry(x).or_default()) += 1;
        acc
    })
}

fn rank_map(p: &[usize]) -> HashMap<usize, usize> {
    let counts = count_presences(p);

    let mut rank_map = HashMap::new();
    let mut rank = 1;

    let mut keys = counts.keys().collect::<Vec<_>>();
    keys.sort_by(|a, b| b.cmp(a));

    for &k in keys {
        rank_map.insert(k, rank);
        rank += counts[&k];
    }
    rank_map
}
