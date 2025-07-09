use proconio::input;

fn main() {
    input! {
        n: usize, x: usize, y: usize,
        mut a: [usize; n],
        mut b: [usize; n],
    }

    a.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));

    let mut count = 0;
    let mut sum_a = 0;
    let mut sum_b = 0;
    for i in 0..n {
        count += 1;
        sum_a += a[i];
        sum_b += b[i];
        if sum_a > x || sum_b > y {
            break;
        }
    }

    println!("{count}")
}
