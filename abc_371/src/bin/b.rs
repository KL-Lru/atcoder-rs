use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _: usize, m: usize,
        p: [(usize, char); m],
    }

    let mut set = HashSet::new();

    for (a, b) in p {
        match b {
            'M' => {
                if set.contains(&a) {
                    println!("No");
                } else {
                    println!("Yes");
                    set.insert(a);
                }
            }
            'F' => println!("No"),
            _ => unreachable!(),
        }
    }
}
