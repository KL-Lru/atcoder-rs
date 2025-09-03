use proconio::input;

fn main() {
    input! {
        x: usize, y: usize,
    }

    println!("{}", ((x - 1 + y) % 12) + 1)
}
