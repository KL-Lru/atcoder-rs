use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut maps = HashMap::new();
    let mut b = Vec::new();

    for (i, ai) in a.iter().enumerate() {
        match maps.get_mut(ai) {
            Some(before) => b.push(*before),
            None => b.push(-1),
        }
        maps.insert(*ai, (i + 1) as isize);
    }

    println!(
        "{}",
        b.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
