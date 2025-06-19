use proconio::{input, marker::Chars};

fn main() {
    input! { mut s: Chars }

    s.sort();
    if s.iter().collect::<String>() == "ABC" {
        println!("Yes");
    } else {
        println!("No");
    }
}
