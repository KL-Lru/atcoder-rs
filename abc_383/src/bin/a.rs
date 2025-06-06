use proconio::input;

fn main() {
    input! {
        n: usize,
        serves: [(usize, usize); n],
    }

    let mut water: usize = 0;
    let mut buf = 0;

    for (t, v) in serves {
        water = water.saturating_sub(t - buf);
        water += v;
        buf = t;
    }

    println!("{water}");
}
