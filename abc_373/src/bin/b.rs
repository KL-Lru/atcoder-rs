use proconio::{input, marker::Chars};

fn main() {
    input! { s: Chars }

    let mut pos = [0isize; 26];

    for (i, si) in s.iter().enumerate() {
        let idx = *si as usize - 'A' as usize;
        pos[idx] = i as isize + 1;
    }

    let mut ans = 0;
    for i in 1..26 {
        ans += (pos[i] - pos[i - 1]).abs();
    }

    println!("{ans}");
}
