use proconio::input;

fn main() {
    input! {
        n: usize
    }

    match n % 2 {
        0 => {
            let rep = (n / 2) - 1;
            println!("{}=={}", "-".repeat(rep), "-".repeat(rep));
        }
        _ => {
            let rep = n / 2;
            println!("{}={}", "-".repeat(rep), "-".repeat(rep));
        }
    }
}
