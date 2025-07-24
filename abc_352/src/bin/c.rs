use proconio::input;

fn main() {
    input! {
        n: usize,
        giants: [(usize, usize); n],
    }

    let mx = giants
        .iter()
        .map(|(a, b)| *b - *a)
        .max()
        .expect("Cannot be empty");
    let sm = giants.iter().map(|(a, _)| *a).sum::<usize>();

    println!("{}", mx + sm);
}
