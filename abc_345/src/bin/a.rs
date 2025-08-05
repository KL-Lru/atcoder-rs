use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let len = s.len();
    if s == ["<".to_string(), "=".repeat(len - 2), ">".to_string()].join("") {
        println!("Yes");
    } else {
        println!("No");
    }
}
