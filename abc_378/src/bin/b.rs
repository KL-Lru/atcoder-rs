use proconio::input;

fn main() {
    input! {
        n: usize,
        rules: [(usize, usize); n],
        q: usize,
    }

    for _ in 0..q {
        input! {
            t: usize, d: usize,
        }
        let (q, r) = rules[t - 1];

        if d % q <= r {
            println!("{}", (d / q) * q + r);
        } else {
            println!("{}", ((d / q) + 1) * q + r);
        }
    }
}
