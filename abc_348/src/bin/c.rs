use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        beans: [(usize, usize); n],
    }

    let mut map = HashMap::<usize, usize>::new();

    for (ai, ci) in beans {
        match map.get(&ci) {
            Some(&x) if x > ai => {
                map.insert(ci, ai);
            }
            Some(_) => {}
            None => {
                map.insert(ci, ai);
            }
        }
    }

    let ans = map.values().max().expect("Cannot empty");

    println!("{ans}");
}
