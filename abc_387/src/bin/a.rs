use proconio::input;

fn main() {
    input! {
      a: usize, b: usize,
    }

    let ans = (a + b).pow(2);
    println!("{ans}");
}
