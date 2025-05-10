use proconio::input;

const TAKAHASHI: usize = 400;

fn main() {
    input! {
      a: usize
    }

    if TAKAHASHI % a == 0 {
        println!("{}", TAKAHASHI / a);
    } else {
        println!("-1");
    }
}
