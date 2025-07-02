use proconio::input;

fn main() {
    input! {
        n: usize,
        op: [(usize, char); n]
    }

    let mut ans = 0;
    let mut l = None;
    let mut r = None;

    for (x, c) in op {
        match (c, l, r) {
            ('L', None, _) => l = Some(x),
            ('L', Some(v), _) => {
                ans += v.abs_diff(x);
                l = Some(x);
            }
            ('R', _, None) => r = Some(x),
            ('R', _, Some(v)) => {
                ans += v.abs_diff(x);
                r = Some(x);
            }
            _ => unreachable!(),
        }
    }

    println!("{ans}");
}
