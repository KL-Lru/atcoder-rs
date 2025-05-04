use proconio::input;

fn main() {
    input! {
      n: usize,
      a: [u32; 2 * n],
    }

    let mut ans = 0;
    for i in 1..(2 * n - 1) {
        if a[i - 1] == a[i + 1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
