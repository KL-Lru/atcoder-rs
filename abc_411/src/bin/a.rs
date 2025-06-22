use proconio::input;

fn main() {
    input! {
        p: String,
        l: usize,
    }

    if p.len() >= l {
        println!("Yes");
    } else {
        println!("No");
    }
}
