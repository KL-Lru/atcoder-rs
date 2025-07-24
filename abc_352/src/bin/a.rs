use proconio::input;

fn main() {
    input! {
        _: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    if (x.min(y)..=y.max(x)).contains(&z) {
        println!("Yes");
    } else {
        println!("No");
    }
}
