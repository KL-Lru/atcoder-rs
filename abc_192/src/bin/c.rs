use itertools::Itertools;
use proconio::input;

fn g1(num: i64) -> i64 {
    num.to_string()
        .chars()
        .sorted_by(|a, b| b.cmp(a))
        .collect::<String>()
        .parse()
        .unwrap()
}

fn g2(num: i64) -> i64 {
    num.to_string()
        .chars()
        .sorted()
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    input! {
        mut n: i64,
        k: i64,
    };

    for _ in 0..k {
        n = g1(n) - g2(n);
    }
    println!("{}", n);
}
