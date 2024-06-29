use std::cmp::max;

use proconio::input;

fn main() {
    input! {
      n: u32,
      a: u32,
      t: [u32; n],
    }

    let mut prev = 0;
    for ti in t {
        let end_time = max(prev, ti) + a;
        println!("{}", end_time);
        prev = end_time;
    }
}
