use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let n = s[3..].parse::<usize>().expect("Failed to parse number");

    if 0 < n && n < 350 && n != 316 {
        println!("Yes");
    } else {
        println!("No");
    }
}
