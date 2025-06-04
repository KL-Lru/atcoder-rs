use proconio::input;

fn main() {
    input! {
      a: usize, b: usize, c: usize,
    }

    if (a == b && b == c) || (a + b == c) || (a + c == b) || (b + c == a) {
        println!("Yes");
    } else {
        println!("No");
    }
}
