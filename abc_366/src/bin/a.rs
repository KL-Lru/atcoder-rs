use proconio::input;

fn main() {
    input! { n: usize, t: usize, a: usize }

    if n / 2 < t || n / 2 < a {
        println!("Yes");
    } else {
        println!("No");
    }
}
