use std::collections::HashMap;

use proconio::input;

const MX: usize = 2 * 1e+5 as usize;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let sq = square_num();

    for ai in a.iter_mut() {
        for &si in sq.iter() {
            if *ai % si == 0 {
                *ai /= si;
            }
        }
    }

    let mut map = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0usize) += 1;
    }

    let mut ans = 0;

    for (k, v) in map {
        if k == 0 {
            ans += v * (n - v);
        }
        ans += v * (v - 1) / 2;
    }

    println!("{ans}");
}

fn square_num() -> Vec<usize> {
    let mut result = vec![];
    let mut buf = 1usize;

    while result.last().is_none() || result.last() < Some(&MX) {
        result.push(buf.pow(2));
        buf += 1;
    }

    result.reverse();
    result
}
