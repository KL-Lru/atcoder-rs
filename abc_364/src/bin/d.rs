use proconio::input;
use superslice::Ext;

const INF: isize = 1e+9 as isize;

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [isize; n],
        bk: [(isize, usize); q],
    }

    a.sort();

    for (b, k) in bk {
        if count_around(&a, b, 0) >= k {
            println!("0");
            continue;
        }

        let mut l = 0;
        let mut r = INF;

        while r - l > 1 {
            let m = (l + r) / 2;

            if count_around(&a, b, m) >= k {
                r = m;
            } else {
                l = m;
            }
        }

        println!("{r}");
    }
}

fn count_around(a: &[isize], base: isize, d: isize) -> usize {
    let upper_threshold = base + d;
    let lower_threshold = base - d;
    let lower_bound = a.lower_bound(&lower_threshold);
    let upper_bound = a.upper_bound(&upper_threshold);

    if upper_bound == a.len() - 1 && a[upper_bound] == upper_threshold {
        upper_bound - lower_bound + 1
    } else {
        upper_bound - lower_bound
    }
}
