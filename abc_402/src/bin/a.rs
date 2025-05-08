use proconio::input;

fn main() {
    input! {
        s: String
    }

    println!(
        "{}",
        s.chars().filter(|&c| c.is_uppercase()).collect::<String>()
    );
}
