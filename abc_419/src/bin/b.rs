use proconio::input;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    input! { q: usize }

    let mut pq = BinaryHeap::new();

    for _ in 0..q {
        input! { t: usize }

        match t {
            1 => {
                input! {x: usize }
                pq.push(Reverse(x));
            }
            2 => {
                if let Some(Reverse(x)) = pq.pop() {
                    println!("{x}");
                }
            }
            _ => unreachable!(),
        }
    }
}
