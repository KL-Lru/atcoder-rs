use proconio::input;

fn main() {
    input! {
        n: usize,
        tasks: [(usize, usize); n],
    }

    println!("{}", tasks.iter().filter(|&(a, b)| a < b).count());
}
