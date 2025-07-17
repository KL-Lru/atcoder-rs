use proconio::input;

fn main() {
    input! {
        s: String
    }

    let upper = s.chars().filter(|c| c.is_uppercase()).count();
    let lower = s.chars().filter(|c| c.is_lowercase()).count();

    if upper > lower {
        println!("{}", s.to_uppercase());
    } else {
        println!("{}", s.to_lowercase());
    }
}
