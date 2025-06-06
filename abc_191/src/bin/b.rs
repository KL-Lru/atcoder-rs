use proconio::input;

fn main() {
    input! {
        n: usize,
        x: u64,
        a: [u64; n],
    }
    for ai in a {
        if ai != x {
            print!("{} ", ai);
        }
    }
    println!();
}
