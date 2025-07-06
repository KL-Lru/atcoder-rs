use std::collections::HashMap;

use proconio::input;

fn main() {
    input! { q: usize }

    let mut map = HashMap::new();

    for _ in 0..q {
        input! { t: usize }

        match t {
            1 => {
                input! { x: usize }
                map.entry(x).and_modify(|v| *v += 1).or_insert(1);
            }
            2 => {
                input! { x: usize }
                match map.get_mut(&x) {
                    Some(v) if *v > 1 => {
                        *v -= 1;
                    }
                    Some(_) => {
                        map.remove(&x);
                    }
                    None => unreachable!(),
                }
            }
            3 => {
                println!("{}", map.len());
            }
            _ => unreachable!(),
        }
    }
}
