use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = 0;
    for &ai in a.iter() {
        let idx = binary_search(&a, ai * 2);
        ans += n - idx - 1;
    }

    println!("{ans}");
}

fn binary_search(a: &[usize], x: usize) -> usize {
    let mut l = 0;
    let mut r = a.len();
    while (r - l) > 1 {
        let m = (l + r) / 2;
        if a[m] < x {
            l = m;
        } else {
            r = m;
        }
    }
    l
}
