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
        mut a: [isize; n],
    }

    if n < 3 {
        println!("Yes");
        return;
    }

    if is_geometric_1(&a) || is_geometric_neg_1(&a) {
        println!("Yes");
        return;
    }

    a.sort_by_key(|a| a.abs());

    for i in 2..n {
        if a[i - 1] * a[i - 1] != a[i] * a[i - 2] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn is_geometric_1(a: &[isize]) -> bool {
    HashSet::<isize>::from_iter(a.iter().cloned()).len() == 1
}

fn is_geometric_neg_1(a: &[isize]) -> bool {
    let f = a[0];
    let fc = a.iter().filter(|&&x| x == f).count();
    let fnc = a.iter().filter(|&&x| x == -f).count();

    fc + fnc == a.len() && fc.abs_diff(fnc) <= 1
}
