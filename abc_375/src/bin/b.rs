use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [(isize, isize); n]
    }

    let mut current = (0, 0);
    let mut ans = 0.0;

    for (x, y) in p {
        let dx = (x - current.0).abs();
        let dy = (y - current.1).abs();
        ans += ((dx.pow(2) + dy.pow(2)) as f64).sqrt();
        current = (x, y);
    }

    ans += ((current.0.pow(2) + current.1.pow(2)) as f64).sqrt();

    println!("{:.10}", ans);
}
