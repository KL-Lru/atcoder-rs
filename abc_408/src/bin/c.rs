use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        r: [(usize, usize);m],
    }

    let mut covering = vec![0; n + 2];

    for (l, r) in r {
        covering[l] += 1;
        if r < n {
            covering[r + 1] -= 1;
        }
    }
    for i in 1..=n {
        covering[i] += covering[i - 1];
    }

    let ans = covering.into_iter().skip(1).take(n).min().unwrap_or(0);
    println!("{ans}");
}
