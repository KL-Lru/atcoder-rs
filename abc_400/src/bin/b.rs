use proconio::input;

const THRESHOLD: u64 = 1e+9 as u64;

fn main() {
    input! {
        n: u64, m: u32,
    }

    let mut x = 0;
    for i in 0..=m {
        x += n.pow(i);
        if x > THRESHOLD {
            println!("inf");
            return;
        }
    }

    println!("{x}");
}
