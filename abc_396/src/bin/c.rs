use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut b: [i64; n],
        mut w: [i64; m],
    }

    desc_sort(&mut b);
    desc_sort(&mut w);

    let partial_sum_b = partial_sum(&b);
    let partial_sum_w = partial_sum(&w);

    let max_w = max_before(&partial_sum_w);

    let mut ans = 0;
    for i in 0..=n {
        ans = ans.max(partial_sum_b[i] + max_w[i.min(m)]);
    }

    println!("{ans}");
}

fn desc_sort<T: Ord>(arr: &mut [T]) {
    arr.sort_by(|a, b| b.cmp(a));
}

fn partial_sum(arr: &[i64]) -> Vec<i64> {
    let mut result = vec![0];
    arr.iter().fold(0, |acc, &x| {
        result.push(acc + x);
        acc + x
    });
    result
}

fn max_before(arr: &[i64]) -> Vec<i64> {
    arr.iter().fold(Vec::new(), |mut acc, &x| {
        if let Some(&last) = acc.last() {
            acc.push(x.max(last));
        } else {
            acc.push(x);
        }
        acc
    })
}
