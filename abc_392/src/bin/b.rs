use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
    }

    let mut set = HashSet::new();
    for ai in a {
        set.insert(ai);
    }

    let mut ans = Vec::new();
    for i in 1..=n {
        if !set.contains(&i) {
            ans.push(i);
        }
    }

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
