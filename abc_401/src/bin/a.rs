use proconio::input;

fn main() {
    input! {
      s: usize,
    }

    match s {
        200..=299 => println!("Success"),
        _ => println!("Failure"),
    }
}
