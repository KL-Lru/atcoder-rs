use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; n],
        b: [usize; q],
    }

    let max_a = *a.iter().max().expect("Not Empty");

    let mut map = HashMap::new();
    for ai in a {
        *map.entry(ai).or_insert(0usize) += 1;
    }

    let mut pv = vec![1];

    let mut buf = n;
    for i in 1..=max_a {
        let prev = pv.last().unwrap_or(&0);
        pv.push(prev + buf);
        buf -= map.get(&i).unwrap_or(&0);
    }

    for bi in b {
        if bi > max_a {
            println!("-1");
        } else {
            println!("{}", pv[bi - 1]);
        }
    }
}
