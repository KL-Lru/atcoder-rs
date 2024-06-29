use std::cmp::{max, min};

use proconio::input;

fn main() {
    input! {
      h: usize, w: usize, k: u64,
      si: usize, sj: usize,
      a: [[u64; w]; h],
    }

    let movement_turns = min(k, 2500u64) as usize;

    // db[turn count][X][Y] = max enjoyment
    let mut dp = vec![vec![vec![0u64; w]; h]; movement_turns];

    for i in 0..movement_turns {
        if i == 0 {
            for x in 0..h {
                for y in 0..w {
                    if (x.abs_diff(si - 1) <= 1 && y == (sj - 1)) || (y.abs_diff(sj - 1) <= 1 && x == (si - 1)) {
                        dp[i][x][y] = a[x][y];
                    } else {
                        dp[i][x][y] = 0;
                    }
                }
            }
        } else {
            for x in 0..h {
                for y in 0..w {
                    let dx = [0, 1, 0, -1];
                    let dy = [1, 0, -1, 0];

                    let mut prev_max = 0;
                    for d in 0..4 {
                        let xi = x as i64 + dx[d];
                        let yi = y as i64 + dy[d];

                        if xi >= 0 && xi < h as i64 && yi >= 0 && yi < w as i64 {
                            prev_max = max(prev_max, dp[i - 1][xi as usize][yi as usize])
                        }
                    }

                    // cannot reach to this point
                    if prev_max == 0 && dp[i - 1][x][y] == 0 {
                        continue;
                    }

                    dp[i][x][y] = max(prev_max, dp[i - 1][x][y]) + a[x][y];
                }
            }
        }
    }

    let mut ans = 0;
    for x in 0..h {
        for y in 0..w {

            ans = max(
                ans,
                dp[movement_turns - 1][x][y] + a[x][y] * (k - movement_turns as u64),
            );
        }
    }

    println!("{}", ans);
}
