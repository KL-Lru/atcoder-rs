use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n]
    }

    let mut presence = HashSet::new();
    for (i, ai) in a.iter().enumerate() {
        presence.insert(ai);
        if presence.len() == m {
            println!("{}", n - i);
            return;
        }
    }

    println!("0");
}
