use proconio::input;

fn main() {
    input! {
        s: String,
    };

    for (i, c) in s.chars().enumerate() {
        if (i % 2 == 0 && c.is_uppercase()) || (i % 2 == 1 && c.is_lowercase()) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
