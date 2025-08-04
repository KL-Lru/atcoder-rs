use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    let uniq_a = HashSet::<usize>::from_iter(a)
        .into_iter()
        .collect::<Vec<_>>();
    let sm = uniq_a.iter().filter(|&&x| x <= k).sum::<usize>();

    println!("{}", (k * (k + 1) / 2) - sm);
}
