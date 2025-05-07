use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        _: usize,
        _: usize,
        q: usize,
    }

    let mut authority_map = HashMap::<usize, HashSet<usize>>::new();
    let mut all_authority = HashSet::<usize>::new();

    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! { x: usize, y: usize }
                authority_map.entry(x).or_default().insert(y);
            }
            2 => {
                input! { x: usize }
                all_authority.insert(x);
            }
            3 => {
                input! { x: usize, y: usize }
                if all_authority.contains(&x) || authority_map.entry(x).or_default().contains(&y) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
