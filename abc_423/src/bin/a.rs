use proconio::input;

fn main() {
    input! {
      x: usize, c: usize,
    }

    println!("{}", (x / (1000 + c) * 1000));
}
