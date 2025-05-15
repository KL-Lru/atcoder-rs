use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut forward_counts = Vec::new();
    let mut backward_counts = Vec::new();

    let mut s = HashSet::new();
    for ai in a.iter() {
        s.insert(ai);
        forward_counts.push(s.len());
    }
    s.clear();
    for ai in a.iter().rev() {
        s.insert(ai);
        backward_counts.push(s.len());
    }
    backward_counts.reverse();

    let mut ans = 0;
    for i in 0..(n - 1) {
        ans = ans.max(forward_counts[i] + backward_counts[i + 1]);
    }

    println!("{ans}");
}
