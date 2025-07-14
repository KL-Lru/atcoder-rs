use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        mut a: [usize; n],
    }

    let rem = n - k - 1;
    a.sort();

    let mut ans = 1e+10 as usize;
    for i in 0..(n - rem) {
        ans = ans.min(a[i + rem] - a[i]);
    }

    println!("{ans}");
}
