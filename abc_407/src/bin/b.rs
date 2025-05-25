use proconio::input;

fn main() {
    input! {
        x: usize, y: usize,
    }

    let mut cnt = 0usize;
    for a in 1..=6 {
        for b in 1..=6 {
            if a + b >= x || a.abs_diff(b) >= y {
                cnt += 1;
            }
        }
    }

    println!("{}", (cnt as f64) / 36f64);
}
