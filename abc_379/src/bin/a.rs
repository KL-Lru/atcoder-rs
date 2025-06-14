use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: Chars,
    }

    println!(
        "{} {}",
        [n[1], n[2], n[0]].iter().collect::<String>(),
        [n[2], n[0], n[1]].iter().collect::<String>()
    );
}
