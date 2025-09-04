use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
        x: usize, y: String,
    }

    if s[x - 1] == y {
        println!("Yes");
    } else {
        println!("No");
    }
}
