use proconio::input;

fn main() {
    input! {
      n: usize,
      m: usize,
      mut a: [u64; n],
      mut b: [u64; m],
    }

    a.sort();
    b.sort();

    let mut ans = 0;
    let mut idx = 0;

    for bi in b {
        while idx < a.len() && a[idx] < bi {
            idx += 1;
        }
        if idx >= a.len() {
            println!("-1");
            return;
        }
        ans += a[idx];
        idx += 1;
    }

    println!("{}", ans);
}
