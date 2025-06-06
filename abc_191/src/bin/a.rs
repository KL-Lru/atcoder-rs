use proconio::input;

fn main() {
    input! {
      v: u64,
      t: u64,
      s: u64,
      d: u64,
    }

    if v * t <= d && d <= v * s {
        println!("No");
    } else {
        println!("Yes");
    }
}
