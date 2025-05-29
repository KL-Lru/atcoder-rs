use core::panic;

use proconio::input;

fn main() {
    input! { q: usize }

    let mut tail = 0;
    let mut head_idx = 0;
    let mut queue = vec![];

    for _ in 0..q {
        input! { t: usize }

        match t {
            1 => {
                input! { l: usize }
                queue.push(tail);
                tail += l;
            }
            2 => {
                head_idx += 1;
            }
            3 => {
                input! { k: usize }
                println!("{}", queue[head_idx + k - 1] - queue[head_idx]);
            }
            _ => panic!("Invalid input"),
        }
    }
}
