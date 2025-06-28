use std::collections::HashSet;

use proconio::input;

fn main() {
    input! { t: usize }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: usize,
        s: [usize; n],
    }

    let first = s[0];
    let last = s[n - 1];

    if last <= first * 2 {
        println!("2");
        return;
    }

    let mut dominoes =
        HashSet::<usize>::from_iter(s.into_iter().filter(|&si| first <= si && si <= last))
            .into_iter()
            .collect::<Vec<_>>();
    dominoes.sort();

    let mut arr = vec![first];
    let mut buf = None;
    for &d in dominoes.iter().skip(1) {
        if let Some(&l) = arr.last() {
            if l * 2 < d {
                if let Some(b) = buf {
                    if l * 2 >= b {
                        arr.push(b);
                    } else {
                        println!("-1");
                        return;
                    }
                } else {
                    println!("-1");
                    return;
                }
            }
        }
        buf = Some(d);
    }

    if let Some(&l) = arr.last() {
        if l != last {
            if l * 2 >= last {
                arr.push(last);
            } else {
                println!("-1");
                return;
            }
        }
    }

    println!("{}", arr.len());
}
