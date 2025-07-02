use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    }

    if a == b {
        println!("1");
    } else if (a.abs_diff(b) % 2) == 1 {
        println!("2");
    } else {
        println!("3");
    }
}
