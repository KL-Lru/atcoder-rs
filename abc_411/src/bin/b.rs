use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [usize; n-1],
    }

    let sum = d.iter().fold(vec![0], |mut acc, &x| {
        if let Some(last) = acc.last() {
            acc.push(last + x);
        } else {
            acc.push(x);
        }
        acc
    });

    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            print!("{} ", sum[j] - sum[i]);
        }
        println!();
    }
}
