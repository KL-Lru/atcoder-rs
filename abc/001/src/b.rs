//! refs: https://atcoder.jp/contests/abc001/tasks/abc001_2

use proconio::input;

fn main() {
    input! {
        m: i32,
    }

    let ans = if m < 100 {
        0
    } else if m <= 5000 {
        m / 100
    } else if m <= 30000 {
        m / 1000 + 50
    } else if m <= 70000 {
        (m / 1000 - 30) / 5 + 80
    } else {
        89
    };

    println!("{}", ans);
}
