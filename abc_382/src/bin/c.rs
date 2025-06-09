use proconio::input;

const INF: usize = 1 << 60;

fn main() {
    input! { n: usize, m: usize }

    let mut min = INF;
    let mut a = vec![(INF, -1)];

    for idx in 1..=n {
        input! { ai: usize }
        if ai < min {
            a.push((ai, idx as isize));
            min = ai;
        }
    }
    a.push((0, -1));

    for _ in 0..m {
        input! { bi: usize }

        let mut l = 0;
        let mut r = a.len() - 1;
        while r - l > 1 {
            let mid = (l + r) / 2;
            if a[mid].0 > bi {
                l = mid;
            } else {
                r = mid;
            }
        }

        println!("{}", a[r].1);
    }
}
