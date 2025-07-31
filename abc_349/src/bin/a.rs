use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n - 1],
    }

    println!("{}", -a.iter().sum::<isize>());
}
