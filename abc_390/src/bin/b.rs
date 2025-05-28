use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if n <= 2 {
        println!("Yes");
        return;
    }

    for i in 1..(n - 1) {
        if a[i] * a[i] != a[i - 1] * a[i + 1] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
