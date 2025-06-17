use itertools::Itertools;
use proconio::input;

fn main() {
    input! { mut a: [usize; 4] }
    a.sort();

    let count = a
        .into_iter()
        .group_by(|&x| x)
        .into_iter()
        .map(|(_, group)| group.count() / 2)
        .sum::<usize>();

    println!("{count}");
}
