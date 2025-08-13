use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut map = HashMap::new();
    let mut ans = 0isize;

    for (i, ai) in a.iter().enumerate() {
        if let Some(x) = map.get(&(i as isize - ai)) {
            ans += x;
        }

        *map.entry(i as isize + ai).or_insert(0) += 1;
    }

    println!("{ans}");
}
