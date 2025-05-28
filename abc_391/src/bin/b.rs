use proconio::{input, marker::Chars};

fn main() {
    input! {n: usize, m: usize}
    input! {s: [Chars; n]}
    input! {t: [Chars; m]}

    for i in 0..(n - m + 1) {
        for j in 0..(n - m + 1) {
            if satisfy((i, j), &s, &t) {
                println!("{} {}", i + 1, j + 1);
                return;
            }
        }
    }
}

fn satisfy(point: (usize, usize), s: &[Vec<char>], t: &[Vec<char>]) -> bool {
    let (x, y) = point;

    let n = t.len();

    for i in 0..n {
        for j in 0..n {
            if s[x + i][y + j] != t[i][j] {
                return false;
            }
        }
    }

    true
}
