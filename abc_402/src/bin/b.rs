use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
      q: usize,
    }

    let mut queue = VecDeque::new();
    for _ in 0..q {
        input! {
            op: usize,
        }
        match op {
            1 => {
                input! {
                    x: usize,
                }
                queue.push_back(x);
            }
            2 => {
                let x = queue.pop_front().expect("Cannot occur due to constraints");
                println!("{}", x);
            }
            _ => {}
        }
    }
}
