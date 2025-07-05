use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut st = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            st.insert(s[i].clone() + &s[j]);
        }
    }

    println!("{}", st.len());
}
