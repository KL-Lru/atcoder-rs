use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }

    println!("{}", a.iter().filter(|&&x| x >= k).count());
}
