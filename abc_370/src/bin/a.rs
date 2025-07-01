use proconio::input;

fn main() {
    input! { l: usize, r: usize }

    match (l, r) {
        (1, 0) => println!("Yes"),
        (0, 1) => println!("No"),
        _ => println!("Invalid"),
    }
}
