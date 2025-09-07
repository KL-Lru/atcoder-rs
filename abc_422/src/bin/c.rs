use proconio::input;

const INF: usize = 1e+9 as usize;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        na: usize, nb: usize, nc: usize,
    }

    let mut l = 0;
    let mut r = INF + 1;

    while (r - l) > 1 {
        let mid = (r + l) / 2;

        if (na < mid) || (nc < mid) || (na - mid) + (nc - mid) + nb < mid {
            r = mid;
            continue;
        }

        l = mid;
    }

    println!("{l}");
}
