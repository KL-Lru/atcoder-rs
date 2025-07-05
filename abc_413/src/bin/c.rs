use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! { q: usize }

    let mut queue = VecDeque::new();

    for _ in 0..q {
        input! { t: usize }
        match t {
            1 => {
                input! { c: usize, x: usize }
                queue.push_back((c, x));
            }
            2 => {
                input! { k: usize }
                let mut pc = 0;
                let mut sm = 0;

                while pc < k {
                    if let Some((c, x)) = queue.pop_front() {
                        if c <= k - pc {
                            sm += c * x;
                            pc += c;
                        } else {
                            let pop = k - pc;
                            queue.push_front((c - pop, x));
                            sm += pop * x;
                            pc += pop;
                        }
                    }
                }

                println!("{}", sm);
            }
            _ => unreachable!(),
        }
    }
}
