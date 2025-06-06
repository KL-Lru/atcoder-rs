use proconio::input;

fn main() {
    input! {
        x: i64,
    };

    println!("{}", 100 - (x % 100));
}
