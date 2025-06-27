use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, q: usize,
        mut s: Chars,
        queries: [(usize, char); q],
    }

    let mut ans = 0;
    for i in 0..(n - 2) {
        if s[i..=(i + 2)] == ['A', 'B', 'C'] {
            ans += 1;
        }
    }

    for (x, c) in queries {
        let idx = x - 1;
        if 2 <= idx {
            match ((s[idx - 2], s[idx - 1], s[idx]), c) {
                (('A', 'B', 'C'), 'C') => {}
                (('A', 'B', 'C'), _) => ans -= 1,
                (('A', 'B', _), 'C') => ans += 1,
                _ => {}
            }
        }
        if 1 <= idx && idx < n - 1 {
            match ((s[idx - 1], s[idx], s[idx + 1]), c) {
                (('A', 'B', 'C'), 'B') => {}
                (('A', 'B', 'C'), _) => ans -= 1,
                (('A', _, 'C'), 'B') => ans += 1,
                _ => {}
            }
        }
        if idx < n - 2 {
            match ((s[idx], s[idx + 1], s[idx + 2]), c) {
                (('A', 'B', 'C'), 'A') => {}
                (('A', 'B', 'C'), _) => ans -= 1,
                ((_, 'B', 'C'), 'A') => ans += 1,
                _ => {}
            }
        }
        s[idx] = c;
        println!("{ans}");
    }
}
