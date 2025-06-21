use proconio::input;

fn main() {
    input! {
        n: usize, c: isize,
        t: [isize; n]
    }

    let mut buf = 0 - c;
    let mut ans = 0;

    for ti in t {
        if ti - buf >= c {
            buf = ti;
            ans += 1;
        }
    }

    println!("{ans}");
}
