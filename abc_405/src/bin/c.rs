use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
    }

    let sum_vec = a.iter().fold(vec![], |mut acc, &x| match acc.last() {
        Some(&last) => {
            acc.push(last + x);
            acc
        }
        None => {
            acc.push(x);
            acc
        }
    });

    let mut ans = 0;
    for i in 1..n {
        ans += sum_vec[i - 1] * a[i];
    }

    println!("{ans}");
}
