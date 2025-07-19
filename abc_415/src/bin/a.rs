use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        x: usize,
    }

    if HashSet::<usize>::from_iter(a).contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
