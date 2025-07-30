use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        t: [usize; q],
    }

    let mut teeth = vec![true; n];

    for ti in t {
        teeth[ti - 1] = !teeth[ti - 1];
    }

    println!(
        "{}",
        teeth.iter().filter(|&&x| x).count()
    )
}
