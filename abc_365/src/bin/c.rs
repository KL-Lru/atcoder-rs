use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }

    if m >= a.iter().sum() {
        println!("infinite");
        return;
    }

    let mut l = 0;
    let mut r = m;

    while r - l > 1 {
        let mid = (l + r) / 2;
        let sum = a.iter().map(|&x| x.min(mid)).sum::<usize>();

        if sum > m {
            r = mid;
        } else {
            l = mid;
        }
    }

    println!("{l}");
}
