use proconio::input;

fn main() {
    input! {
      a: usize, b: usize, c: usize
    }

    if a == b || b == c || c == a {
        println!("Yes");
    } else {
        println!("No");
    }
}
