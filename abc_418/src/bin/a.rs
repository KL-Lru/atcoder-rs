use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
    }

    if s.ends_with("tea") {
        println!("Yes");
    } else {
        println!("No");
    }
}
