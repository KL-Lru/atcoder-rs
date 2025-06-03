use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        x: [usize; 4]
    }
    let set = HashSet::<usize>::from_iter(x);

    if set.len() == 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}
