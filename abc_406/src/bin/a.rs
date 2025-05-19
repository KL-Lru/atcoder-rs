use proconio::input;

fn main() {
    input! {
      a: usize, b: usize,
      c: usize, d: usize
    }

    if a > c {
        println!("Yes");
    } else if a < c {
        println!("No");
    } else if b > d {
        println!("Yes");
    } else {
        println!("No");
    }
}
