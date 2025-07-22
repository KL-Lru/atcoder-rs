use proconio::input;

fn main() {
  input! { h: usize }

  let mut plant = 0;
  let mut i = 0;

  while plant <= h {
      plant += 2usize.pow(i);
      i += 1;
  }

  println!("{i}");
}
