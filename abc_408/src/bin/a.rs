use proconio::input;

fn main() {
    input! {
        n: usize, s: usize,
        t: [usize; n],
    }

    if t[0] > s {
        println!("No");
        return;
    }

    for i in 1..n {
        if t[i] - t[i - 1] > s {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
