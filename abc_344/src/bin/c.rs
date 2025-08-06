use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        l: usize,
        c: [usize; l],
        q: usize,
        x: [usize; q],
    }

    let mut answerable = HashSet::new();

    for &ai in a.iter() {
        for &bi in b.iter() {
            for &ci in c.iter() {
                answerable.insert(ai + bi + ci);
            }
        }
    }

    for xi in x {
        if answerable.contains(&xi) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
