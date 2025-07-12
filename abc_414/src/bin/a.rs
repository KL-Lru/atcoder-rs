use proconio::input;

fn main() {
    input! {
        n: usize, l: usize, r: usize,

        p: [(usize, usize); n],
    }

    let mut ans = 0;
    for (x, y) in p {
        if x <= l && r <= y {
            ans += 1;
        }
    }

    println!("{ans}");
}
