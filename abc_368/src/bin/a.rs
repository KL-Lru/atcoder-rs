use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    let mut dq = VecDeque::from(a);

    for _ in 0..k {
        if let Some(last) = dq.pop_back() {
            dq.push_front(last);
        }
    }

    println!(
        "{}",
        dq.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
