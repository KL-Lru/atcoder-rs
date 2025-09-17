use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize, r: usize,
        l: [usize; n],
    }

    let mut q = VecDeque::from_iter(l.into_iter().enumerate());
    while let Some((idx, v)) = q.front() {
        if *v == 0 || *idx == r {
            break;
        }
        q.pop_front();
    }

    while let Some((idx, v)) = q.back() {
        if *v == 0 || *idx + 1 == r {
            break;
        }
        q.pop_back();
    }

    println!("{}", q.len() + q.iter().map(|t| t.1).sum::<usize>());
}
