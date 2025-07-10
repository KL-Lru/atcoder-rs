use proconio::input;

fn main() {
    input! { r: usize }

    println!("{}", 100 - (r % 100));
}
