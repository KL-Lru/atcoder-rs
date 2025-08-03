use proconio::input;

fn main() {
    input! {
        n: usize, a: usize, b: usize,
        s: String,
    }

    println!("{}", &s[a..(n - b)]);
}
