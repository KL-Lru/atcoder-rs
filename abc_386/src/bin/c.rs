use proconio::{input, marker::Chars};

const INF: usize = 1 << 60;

fn main() {
    input! {
        k: usize,
        s: Chars,
        t: Chars,
    }

    if s.len().abs_diff(t.len()) > k {
        println!("No");
        return;
    }

    let distance = levenshtein_distance(k, &s, &t);

    if distance <= k {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn levenshtein_distance(k: usize, s: &[char], t: &[char]) -> usize {
    let n = s.len();
    let m = t.len();
    let mut dp = vec![vec![INF; 2 * k + 1]; n + 1];

    for i in 0..=k.min(n) {
        dp[i][k - i] = i;
    }
    for d in 0..=k.min(m) {
        dp[0][d + k] = d;
    }

    for i in 1..=n {
        for d in 0..=(2 * k) {
            let j = i as isize + d as isize - k as isize;
            if j < 0 || j > m as isize {
                continue;
            }

            let j = j as usize;
            if d < 2 * k {
                dp[i][d] = dp[i][d].min(dp[i - 1][d + 1] + 1); // 削除
            }

            if d > 0 {
                dp[i][d] = dp[i][d].min(dp[i][d - 1] + 1); // 挿入
            }

            if j > 0 {
                let cost = if s[i - 1] == t[j - 1] { 0 } else { 1 };
                dp[i][d] = dp[i][d].min(dp[i - 1][d] + cost); // 置換
            }
        }
    }

    dp[n][m + k - n]
}
