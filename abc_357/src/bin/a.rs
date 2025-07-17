use proconio::input;

fn main() {
    input! {
        n: usize, mut m: usize,
        h: [usize; n],
    }

    let mut sm = 0;

    for i in 0..n {
        sm += h[i];
        if sm > m {
            println!("{i}");
            return;
        }
    }

    println!("{n}")
}
