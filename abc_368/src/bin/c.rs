use proconio::input;

fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }

    let mut t = 1;
    for mut hi in h {
        while t % 3 != 1 && hi > 0 {
            if t % 3 == 0 {
                hi = hi.saturating_sub(3);
            } else {
                hi = hi.saturating_sub(1);
            }
            t += 1;
        }

        t += (hi / 5) * 3;
        hi %= 5;

        while hi > 0 {
            if t % 3 == 0 {
                hi = hi.saturating_sub(3);
            } else {
                hi = hi.saturating_sub(1);
            }
            t += 1;
        }
    }

    println!("{}", t - 1);
}
