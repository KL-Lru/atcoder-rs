use proconio::input;

const INF: usize = 1e+9 as usize;

fn main() {
    input! {
        n: usize,
        k: [usize; n]
    }

    let mut ans = INF;

    for i in 0..(1 << n) {
        let mut sum_a = 0;
        let mut sum_b = 0;

        for j in 0..n {
            if ((i >> j) & 1) == 1 {
                sum_a += k[j];
            } else {
                sum_b += k[j];
            }
        }

        ans = ans.min(sum_a.max(sum_b));
    }

    println!("{ans}");
}
