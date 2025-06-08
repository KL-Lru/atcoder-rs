use proconio::input;

fn main() {
    input! {
        n: usize,
        t: String,
        a: String,
    }

    for i in 0..n {
        if t.chars().nth(i) == Some('o') && a.chars().nth(i) == Some('o') {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
