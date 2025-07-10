use proconio::input;

fn main() {
    input! {
        n: usize, t: usize, p: usize,
        mut l: [usize; n],
    }

    l.sort_by(|a, b| b.cmp(a));

    println!("{}", t.saturating_sub(l[p - 1]));
}
