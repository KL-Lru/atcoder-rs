use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n]
    }

    let mut ans: Vec<_> = HashSet::<usize>::from_iter(a).into_iter().collect();
    ans.sort();

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
