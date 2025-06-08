use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut l = 0;
    let mut r = 101;

    while r - l > 1 {
        let m = (l + r) / 2;
        let cnt = a.iter().filter(|&&x| x >= m).count();

        if cnt >= m {
            l = m;
        } else {
            r = m;
        }
    }

    println!("{l}");
}
