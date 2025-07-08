use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut va = a
        .iter()
        .enumerate()
        .map(|(i, &x)| (x, i + 1))
        .collect::<Vec<_>>();

    va.sort_by(|a, b| b.cmp(a));

    println!("{}", va[1].1);
}
