use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    println!(
        "{}",
        (1..=n)
            .map(|x| if x % 3 != 0 { "o" } else { "x" })
            .collect::<String>()
    )
}
